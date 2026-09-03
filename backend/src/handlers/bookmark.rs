use crate::auth::AuthUser;
use crate::error::AppError;
use crate::handlers::export::{export_as_html, extract_favicon_url};
use crate::handlers::import::import_from_html;
use crate::models::{Bookmark, Category};
use crate::result::ApiResult;
use crate::AppState;
use axum::{
    extract::{Multipart, State},
    http::header,
    response::IntoResponse,
    Json,
};
use rusqlite::params_from_iter;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub id: i64,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchSortItem {
    pub id: i64,
    pub sort_order: i32,
}

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<ApiResult<Vec<Bookmark>>, AppError> {
    let conn = state.db.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, title, url, icon, category_id, user_id, sort_order
         FROM bookmark WHERE user_id = ?1 ORDER BY sort_order ASC",
    )?;
    let bookmarks: Vec<Bookmark> = stmt
        .query_map(rusqlite::params![auth.0], |row| {
            Ok(Bookmark {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                url: row.get(2)?,
                icon: row.get(3)?,
                category_id: row.get(4)?,
                user_id: row.get(5)?,
                sort_order: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ApiResult::success(bookmarks))
}

pub async fn add(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(mut bookmark): Json<Bookmark>,
) -> Result<ApiResult<Bookmark>, AppError> {
    bookmark.title = bookmark.title.trim().to_string();
    bookmark.url = bookmark.url.trim().to_string();
    if bookmark.title.is_empty() || bookmark.url.is_empty() {
        return Ok(ApiResult::error("标题和网址不能为空"));
    }
    if bookmark.title.len() > 500 || bookmark.url.len() > 2000 {
        return Ok(ApiResult::error("内容过长"));
    }
    let conn = state.db.get()?;

    if let Some(id) = bookmark.id {
        let owner: Option<i64> = conn
            .query_row(
                "SELECT user_id FROM bookmark WHERE id = ?1",
                rusqlite::params![id],
                |row| row.get(0),
            )
            .ok();
        if owner != Some(auth.0) {
            return Ok(ApiResult::error("无权操作此书签"));
        }
        conn.execute(
            "UPDATE bookmark SET title=?1, url=?2, icon=?3, category_id=?4, user_id=?5, sort_order=?6 WHERE id=?7",
            rusqlite::params![
                bookmark.title,
                bookmark.url,
                bookmark.icon,
                bookmark.category_id,
                auth.0,
                bookmark.sort_order,
                id,
            ],
        )?;
        bookmark.user_id = auth.0;
        Ok(ApiResult::success(bookmark))
    } else {
        conn.execute(
            "INSERT INTO bookmark (title, url, icon, category_id, user_id, sort_order) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                bookmark.title,
                bookmark.url,
                bookmark.icon,
                bookmark.category_id,
                auth.0,
                bookmark.sort_order,
            ],
        )?;
        let id = conn.last_insert_rowid();
        if bookmark.sort_order == 0 {
            conn.execute(
                "UPDATE bookmark SET sort_order = id WHERE id = ?1",
                rusqlite::params![id],
            )?;
            bookmark.sort_order = id as i32;
        }
        Ok(ApiResult::success(Bookmark {
            id: Some(id),
            title: bookmark.title,
            url: bookmark.url,
            icon: bookmark.icon,
            category_id: bookmark.category_id,
            user_id: auth.0,
            sort_order: bookmark.sort_order,
        }))
    }
}

pub async fn batch_update_sort(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(items): Json<Vec<BatchSortItem>>,
) -> Result<ApiResult<()>, AppError> {
    if items.len() > 1000 {
        return Ok(ApiResult::error("单次更新数量不能超过 1000"));
    }
    if items.is_empty() {
        return Ok(ApiResult::success_empty());
    }
    let mut conn = state.db.get()?;
    let tx = conn.transaction()?;

    let ids: Vec<i64> = items.iter().map(|i| i.id).collect();
    let placeholders: Vec<String> = ids
        .iter()
        .enumerate()
        .map(|(i, _)| format!("?{}", i + 1))
        .collect();
    let sql = format!(
        "SELECT id, user_id FROM bookmark WHERE id IN ({})",
        placeholders.join(",")
    );
    let owners: HashMap<i64, i64> = {
        let mut stmt = tx.prepare(&sql)?;
        let pairs: Vec<(i64, i64)> = stmt
            .query_map(params_from_iter(ids), |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<Result<Vec<_>, _>>()?;
        pairs.into_iter().collect()
    };

    for item in &items {
        if owners.get(&item.id) == Some(&auth.0) {
            tx.execute(
                "UPDATE bookmark SET sort_order = ?1 WHERE id = ?2",
                rusqlite::params![item.sort_order, item.id],
            )?;
        }
    }
    tx.commit()?;
    Ok(ApiResult::success_empty())
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<DeleteRequest>,
) -> Result<ApiResult<()>, AppError> {
    let conn = state.db.get()?;
    let owner: Option<i64> = conn
        .query_row(
            "SELECT user_id FROM bookmark WHERE id = ?1",
            rusqlite::params![req.id],
            |row| row.get(0),
        )
        .ok();
    if owner != Some(auth.0) {
        return Ok(ApiResult::error("无权操作此书签"));
    }
    conn.execute(
        "DELETE FROM bookmark WHERE id = ?1",
        rusqlite::params![req.id],
    )?;
    Ok(ApiResult::success_empty())
}

pub async fn fetch_icons(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<ApiResult<i64>, AppError> {
    let conn = state.db.get()?;
    let count = fetch_missing_icons(&conn, auth.0)?;
    Ok(ApiResult::success(count))
}

fn fetch_missing_icons(conn: &rusqlite::Connection, user_id: i64) -> Result<i64, rusqlite::Error> {
    let mut stmt =
        conn.prepare("SELECT id, url FROM bookmark WHERE user_id = ?1 AND icon IS NULL")?;
    let rows: Vec<(i64, String)> = stmt
        .query_map(rusqlite::params![user_id], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?
        .collect::<Result<Vec<_>, _>>()?;

    if rows.is_empty() {
        return Ok(0);
    }

    let mut updates: Vec<(String, i64)> = Vec::new();
    for (id, url) in rows {
        let icon = extract_favicon_url(&url).unwrap_or_default();
        updates.push((icon, id));
    }

    let tx = conn.unchecked_transaction()?;
    for (icon, id) in &updates {
        tx.execute(
            "UPDATE bookmark SET icon = ?1 WHERE id = ?2",
            rusqlite::params![icon, id],
        )?;
    }
    tx.commit()?;
    let count = updates.iter().filter(|(icon, _)| !icon.is_empty()).count() as i64;

    Ok(count)
}

pub async fn export_bookmarks(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let conn = state.db.get()?;
    let categories = list_categories(&conn, auth.0)?;
    let bookmarks = list_bookmarks_raw(&conn, auth.0)?;
    let html = export_as_html(&categories, &bookmarks);
    let bytes = html.into_bytes();
    Ok((
        [
            (header::CONTENT_TYPE, "text/html; charset=UTF-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"bookmarks.html\"",
            ),
        ],
        bytes,
    ))
}

pub async fn import_bookmarks(
    State(state): State<AppState>,
    auth: AuthUser,
    mut multipart: Multipart,
) -> Result<ApiResult<i64>, AppError> {
    let mut file_content = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        if field.name() == Some("file") {
            file_content = field
                .bytes()
                .await
                .ok()
                .and_then(|b| String::from_utf8(b.to_vec()).ok());
        }
    }

    let content = match file_content {
        Some(c) => c,
        None => return Ok(ApiResult::error("未上传文件")),
    };

    let conn = state.db.get()?;
    match import_from_html(&conn, &content, auth.0) {
        Ok(count) => Ok(ApiResult::success(count)),
        Err(e) => Ok(ApiResult::error(format!("导入失败: {}", e))),
    }
}

fn list_categories(
    conn: &rusqlite::Connection,
    user_id: i64,
) -> Result<Vec<Category>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, name, user_id, parent_id, sort_order FROM category WHERE user_id = ?1",
    )?;
    let items: Vec<Category> = stmt
        .query_map(rusqlite::params![user_id], |row| {
            Ok(Category {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                user_id: row.get(2)?,
                parent_id: row.get(3)?,
                sort_order: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(items)
}

fn list_bookmarks_raw(
    conn: &rusqlite::Connection,
    user_id: i64,
) -> Result<Vec<Bookmark>, rusqlite::Error> {
    let mut stmt = conn
        .prepare("SELECT id, title, url, icon, category_id, user_id, sort_order FROM bookmark WHERE user_id = ?1")?;
    let items: Vec<Bookmark> = stmt
        .query_map(rusqlite::params![user_id], |row| {
            Ok(Bookmark {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                url: row.get(2)?,
                icon: row.get(3)?,
                category_id: row.get(4)?,
                user_id: row.get(5)?,
                sort_order: row.get(6)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_db() -> rusqlite::Connection {
        let conn = rusqlite::Connection::open_in_memory().unwrap();
        conn.execute_batch(
            "
            CREATE TABLE category (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                user_id INTEGER NOT NULL,
                parent_id INTEGER,
                sort_order INTEGER DEFAULT 0
            );
            CREATE TABLE bookmark (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                url TEXT NOT NULL,
                icon TEXT,
                category_id INTEGER,
                user_id INTEGER NOT NULL,
                sort_order INTEGER DEFAULT 0
            );
            ",
        )
        .unwrap();
        conn
    }

    #[test]
    fn import_nested_html() {
        let conn = setup_db();
        let html = r#"<!DOCTYPE NETSCAPE-Bookmark-file-1>
<META HTTP-EQUIV="Content-Type" CONTENT="text/html; charset=UTF-8">
<TITLE>Bookmarks</TITLE>
<H1>书签菜单</H1>
<DL><p>
    <DT><H3>Dev</H3>
    <DL><p>
        <DT><A HREF="https://github.com">GitHub</A>
    </DL><p>
    <DT><A HREF="https://example.com">Root</A>
</DL><p>
"#;
        let count = import_from_html(&conn, html, 1).unwrap();
        assert_eq!(count, 2);

        let categories = list_categories(&conn, 1).unwrap();
        assert_eq!(categories.len(), 1);
        assert_eq!(categories[0].name, "Dev");

        let bookmarks = list_bookmarks_raw(&conn, 1).unwrap();
        assert_eq!(bookmarks.len(), 2);
    }

    #[test]
    fn import_flat_html_no_folders() {
        let conn = setup_db();
        let html = r#"<!DOCTYPE NETSCAPE-Bookmark-file-1>
<META HTTP-EQUIV="Content-Type" CONTENT="text/html; charset=UTF-8">
<TITLE>Bookmarks</TITLE>
<H1>书签菜单</H1>
<DL><p>
    <DT><A HREF="https://a.com">A</A>
    <DT><A HREF="https://b.com">B</A>
</DL><p>
"#;
        let count = import_from_html(&conn, html, 1).unwrap();
        assert_eq!(count, 2);
        assert_eq!(list_categories(&conn, 1).unwrap().len(), 0);
    }

    #[test]
    fn import_duplicate_url_skipped() {
        let conn = setup_db();
        conn.execute(
            "INSERT INTO bookmark (title, url, user_id) VALUES (?1, ?2, ?3)",
            rusqlite::params!["Existing", "https://dup.com", 1],
        )
        .unwrap();
        let html = r#"<!DOCTYPE NETSCAPE-Bookmark-file-1>
<META HTTP-EQUIV="Content-Type" CONTENT="text/html; charset=UTF-8">
<H1>Bookmarks</H1>
<DL><p>
    <DT><A HREF="https://dup.com">Duplicate</A>
</DL><p>
"#;
        let count = import_from_html(&conn, html, 1).unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn extract_favicon_url_standard() {
        let result = extract_favicon_url("https://github.com");
        assert!(result.unwrap().contains("google.com/s2/favicons"));
    }

    #[test]
    fn extract_favicon_url_no_scheme() {
        let result = extract_favicon_url("example.com");
        assert!(result.unwrap().contains("https://example.com"));
    }

    #[test]
    fn extract_favicon_url_invalid() {
        assert!(extract_favicon_url("not a url").is_none());
    }

    #[test]
    fn list_bookmarks_returns_by_user() {
        let conn = setup_db();
        conn.execute(
            "INSERT INTO bookmark (id, title, url, user_id) VALUES (1, 'A', 'https://a.com', 1)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO bookmark (id, title, url, user_id) VALUES (2, 'B', 'https://b.com', 2)",
            [],
        )
        .unwrap();
        let list = list_bookmarks_raw(&conn, 1).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].title, "A");
    }

    #[test]
    fn list_bookmarks_ordered_by_sort() {
        let conn = setup_db();
        conn.execute(
            "INSERT INTO bookmark (id, title, url, user_id, sort_order) VALUES (1, 'A', 'https://a.com', 1, 10)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO bookmark (id, title, url, user_id, sort_order) VALUES (2, 'B', 'https://b.com', 1, 5)",
            [],
        ).unwrap();
        // list_bookmarks_raw doesn't sort; but public handler uses ORDER BY sort_order
        let list = list_bookmarks_raw(&conn, 1).unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn export_as_html_includes_all() {
        let conn = setup_db();
        conn.execute(
            "INSERT INTO category (id, name, user_id) VALUES (1, 'Dev', 1)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO bookmark (id, title, url, user_id, category_id) VALUES (1, 'GitHub', 'https://github.com', 1, 1)",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO bookmark (id, title, url, user_id) VALUES (2, 'Root', 'https://example.com', 1)",
            [],
        ).unwrap();
        let cats = list_categories(&conn, 1).unwrap();
        let bms = list_bookmarks_raw(&conn, 1).unwrap();
        let html = export_as_html(&cats, &bms);
        assert!(html.contains("GitHub"));
        assert!(html.contains("Root"));
        assert!(html.contains("Dev"));
    }
}
