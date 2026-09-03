use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::extract::State;
use reqwest::Client;
use serde::Serialize;
use tokio::sync::Semaphore;

use crate::auth::AuthUser;
use crate::error::AppError;
use crate::result::ApiResult;
use crate::AppState;

const CONCURRENCY: usize = 20;
const REQUEST_TIMEOUT_SECS: u64 = 3;
const CACHE_TTL_SECS: i64 = 86400;

#[derive(Clone, Debug)]
pub struct CheckProgress {
    pub total: usize,
    pub completed: usize,
    pub results: Vec<CheckResult>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckResult {
    pub id: i64,
    pub title: String,
    pub url: String,
    pub icon: Option<String>,
    pub level: String,
    pub http_code: Option<u16>,
    pub error: Option<String>,
    pub base_url: String,
    pub base_alive: bool,
    pub final_url: Option<String>,
    pub checked_at: i64,
    pub cached: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckStatusResponse {
    total: usize,
    completed: usize,
    finished: bool,
    results: Option<Vec<CheckResult>>,
}

pub async fn check_links(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<ApiResult<String>, AppError> {
    let user_id = auth.0;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let cached = state.check_cache.lock().unwrap().get(&user_id).cloned();

    if let Some((cached_at, cached_results)) = cached {
        if now - cached_at < CACHE_TTL_SECS {
            let mut state_map = state.check_state.lock().await;
            state_map.insert(
                user_id,
                CheckProgress {
                    total: cached_results.len(),
                    completed: cached_results.len(),
                    results: cached_results.clone(),
                },
            );
            return Ok(ApiResult::success("cached".to_string()));
        }
    }

    {
        let mut state_map = state.check_state.lock().await;
        if state_map.contains_key(&user_id) {
            return Ok(ApiResult::error("检测正在进行中"));
        }
        state_map.insert(
            user_id,
            CheckProgress {
                total: 0,
                completed: 0,
                results: vec![],
            },
        );
    }

    let state_clone = state.clone();
    tokio::spawn(async move {
        let results = check_all_links(state_clone.clone(), user_id).await;
        state_clone
            .check_cache
            .lock()
            .unwrap()
            .insert(user_id, (now, results));
    });

    Ok(ApiResult::success("ok".to_string()))
}

pub async fn check_status(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<ApiResult<CheckStatusResponse>, AppError> {
    let mut state_map = state.check_state.lock().await;
    match state_map.get(&auth.0) {
        Some(progress) => {
            let finished = progress.total > 0 && progress.completed >= progress.total;
            let response = CheckStatusResponse {
                total: progress.total,
                completed: progress.completed,
                finished,
                results: if finished {
                    Some(progress.results.clone())
                } else {
                    None
                },
            };
            if finished {
                state_map.remove(&auth.0);
            }
            Ok(ApiResult::success(response))
        }
        None => Ok(ApiResult::success(CheckStatusResponse {
            total: 0,
            completed: 0,
            finished: false,
            results: None,
        })),
    }
}

async fn check_all_links(state: AppState, user_id: i64) -> Vec<CheckResult> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let bookmarks = load_bookmarks(&state, user_id);
    if bookmarks.is_empty() {
        state.check_state.lock().await.remove(&user_id);
        return vec![];
    }

    let total = bookmarks.len();

    {
        let mut state_map = state.check_state.lock().await;
        if let Some(progress) = state_map.get_mut(&user_id) {
            progress.total = total;
        }
    }

    let client = match Client::builder()
        .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
        .user_agent("subnav/0.2.0")
        .redirect(reqwest::redirect::Policy::limited(5))
        .tcp_keepalive(Duration::from_secs(30))
        .pool_max_idle_per_host(4)
        .build()
    {
        Ok(c) => c,
        Err(_) => return vec![],
    };

    let semaphore = Arc::new(Semaphore::new(CONCURRENCY));
    let mut handles = Vec::with_capacity(total);

    for (id, title, url, icon) in bookmarks {
        let client = client.clone();
        let sem = semaphore.clone();
        let state = state.clone();

        handles.push(tokio::spawn(async move {
            let _permit = sem.acquire().await;
            let result = check_single_link(&client, id, &title, &url, icon, now).await;

            let mut state_map = state.check_state.lock().await;
            if let Some(progress) = state_map.get_mut(&user_id) {
                progress.results.push(result);
                progress.completed += 1;
            }
        }));
    }

    for h in handles {
        let _ = h.await;
    }

    let state_map = state.check_state.lock().await;
    state_map
        .get(&user_id)
        .map(|p| p.results.clone())
        .unwrap_or_default()
}

fn load_bookmarks(state: &AppState, user_id: i64) -> Vec<(i64, String, String, Option<String>)> {
    let conn = match state.db.get() {
        Ok(c) => c,
        Err(_) => return vec![],
    };
    let mut stmt =
        match conn.prepare("SELECT id, title, url, icon FROM bookmark WHERE user_id = ?1") {
            Ok(s) => s,
            Err(_) => return vec![],
        };
    let rows = stmt.query_map(rusqlite::params![user_id], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    });
    match rows {
        Ok(r) => r.filter_map(|r| r.ok()).collect(),
        Err(_) => vec![],
    }
}

async fn check_single_link(
    client: &Client,
    id: i64,
    title: &str,
    url_str: &str,
    icon: Option<String>,
    now: i64,
) -> CheckResult {
    let base_url = extract_base_origin(url_str);

    let (level, http_code, error, final_url) = match probe_url(client, url_str).await {
        ProbeResult::Ok { status, final_url } => ("ok".to_string(), Some(status), None, final_url),
        ProbeResult::Suspect {
            status, final_url, ..
        } => (
            "suspect".to_string(),
            Some(status),
            Some("redirected to external domain".to_string()),
            final_url,
        ),
        ProbeResult::Dead {
            status,
            error_msg,
            final_url,
        } => {
            let base_alive = probe_base_alive(client, &base_url).await;
            if base_alive {
                ("page_dead".to_string(), status, Some(error_msg), final_url)
            } else {
                ("site_dead".to_string(), status, Some(error_msg), final_url)
            }
        }
    };

    let base_alive = level != "site_dead" && !base_url.is_empty();

    CheckResult {
        id,
        title: title.to_string(),
        url: url_str.to_string(),
        icon,
        level,
        http_code,
        error,
        base_url,
        base_alive,
        final_url,
        checked_at: now,
        cached: false,
    }
}

enum ProbeResult {
    Ok {
        status: u16,
        final_url: Option<String>,
    },
    Suspect {
        status: u16,
        final_url: Option<String>,
    },
    Dead {
        status: Option<u16>,
        error_msg: String,
        final_url: Option<String>,
    },
}

async fn probe_url(client: &Client, url_str: &str) -> ProbeResult {
    match client.head(url_str).send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let final_url = Some(resp.url().to_string());
            if status < 400 {
                if is_cross_domain(url_str, resp.url().as_str()) {
                    return ProbeResult::Suspect { status, final_url };
                }
                return ProbeResult::Ok { status, final_url };
            }
            ProbeResult::Dead {
                status: Some(status),
                error_msg: format!("HTTP {}", status),
                final_url,
            }
        }
        Err(_) => match client.get(url_str).send().await {
            Ok(resp) => {
                let status = resp.status().as_u16();
                let final_url = Some(resp.url().to_string());
                if is_cross_domain(url_str, resp.url().as_str()) {
                    return ProbeResult::Suspect { status, final_url };
                }
                if status < 400 {
                    return ProbeResult::Ok { status, final_url };
                }
                ProbeResult::Dead {
                    status: Some(status),
                    error_msg: format!("HTTP {}", status),
                    final_url,
                }
            }
            Err(e) => ProbeResult::Dead {
                status: None,
                error_msg: e.to_string(),
                final_url: None,
            },
        },
    }
}

fn is_cross_domain(original: &str, final_url: &str) -> bool {
    let orig = match url::Url::parse(original) {
        Ok(u) => u,
        Err(_) => return false,
    };
    let dest = match url::Url::parse(final_url) {
        Ok(u) => u,
        Err(_) => return false,
    };
    orig.host_str() != dest.host_str()
}

async fn probe_base_alive(client: &Client, base_url: &str) -> bool {
    if base_url.is_empty() {
        return false;
    }
    match client.head(base_url).send().await {
        Ok(resp) => resp.status().as_u16() < 400,
        Err(_) => match client.get(base_url).send().await {
            Ok(resp) => resp.status().as_u16() < 400,
            Err(_) => false,
        },
    }
}

fn extract_base_origin(url_str: &str) -> String {
    match url::Url::parse(url_str) {
        Ok(u) => {
            let host = u.host_str().unwrap_or("");
            if host.is_empty() {
                return String::new();
            }
            if let Some(port) = u.port() {
                format!("{}://{}:{}", u.scheme(), host, port)
            } else {
                format!("{}://{}", u.scheme(), host)
            }
        }
        Err(_) => String::new(),
    }
}
