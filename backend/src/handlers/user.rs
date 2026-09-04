use crate::error::AppError;
use crate::models::User;
use crate::result::ApiResult;
use crate::AppState;
use axum::extract::State;
use axum::Json;

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub id: i64,
    pub username: String,
    pub token: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<ApiResult<LoginResponse>, AppError> {
    let username = req.username.trim();
    if username.is_empty() || req.password.is_empty() {
        return Ok(ApiResult::error("账号或密码不能为空"));
    }
    if username.len() > 50 {
        return Ok(ApiResult::error("用户名过长"));
    }
    if state.login_limiter.is_locked(username) {
        return Ok(ApiResult::error("尝试次数过多，请稍后再试"));
    }
    let conn = state.db.get()?;
    let mut stmt = conn.prepare("SELECT id, username, password FROM user WHERE username = ?1")?;
    let result = stmt.query_row(rusqlite::params![username], |row| {
        Ok(User {
            id: Some(row.get(0)?),
            username: row.get(1)?,
            password: row.get(2)?,
        })
    });
    match result {
        Ok(user) => {
            if bcrypt::verify(&req.password, &user.password).unwrap_or(false) {
                state.login_limiter.clear(username);
                let token =
                    crate::auth::sign_token(&state.jwt_secret, state.config.jwt_expire_hours)
                        .map_err(AppError::internal)?;
                Ok(ApiResult::success(LoginResponse {
                    id: user.id.unwrap(),
                    username: user.username,
                    token,
                }))
            } else {
                state.login_limiter.record_failure(username);
                Ok(ApiResult::error("账号或密码错误"))
            }
        }
        Err(_) => {
            state.login_limiter.record_failure(username);
            Ok(ApiResult::error("账号或密码错误"))
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

pub async fn change_password(
    State(state): State<AppState>,
    auth: crate::auth::AuthUser,
    Json(req): Json<ChangePasswordRequest>,
) -> Result<ApiResult<()>, AppError> {
    if req.new_password.len() < 4 {
        return Ok(ApiResult::error("新密码不能少于 4 位"));
    }
    if req.new_password.len() > 128 {
        return Ok(ApiResult::error("新密码过长"));
    }
    let conn = state.db.get()?;
    let current: String = conn.query_row(
        "SELECT password FROM user WHERE id = ?1",
        rusqlite::params![auth.0],
        |row| row.get(0),
    )?;
    drop(conn);

    if !bcrypt::verify(&req.old_password, &current).unwrap_or(false) {
        return Ok(ApiResult::error("原密码错误"));
    }
    let hashed =
        bcrypt::hash(&req.new_password, 10).map_err(|_| AppError::internal("服务器错误"))?;

    let conn = state.db.get()?;
    conn.execute(
        "UPDATE user SET password = ?1 WHERE id = ?2",
        rusqlite::params![hashed, auth.0],
    )?;
    Ok(ApiResult::success_empty())
}
