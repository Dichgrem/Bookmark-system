use crate::auth::AdminUser;
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
    pub role: String,
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
    let conn = state.db.get()?;
    let mut stmt =
        conn.prepare("SELECT id, username, password, role FROM user WHERE username = ?1")?;
    let result = stmt.query_row(rusqlite::params![username], |row| {
        Ok(User {
            id: Some(row.get(0)?),
            username: row.get(1)?,
            password: row.get(2)?,
            role: row.get(3)?,
        })
    });
    match result {
        Ok(user) => {
            if bcrypt::verify(&req.password, &user.password).unwrap_or(false) {
                let token = crate::auth::sign_token(
                    user.id.unwrap(),
                    &user.role,
                    &state.jwt_secret,
                    state.config.jwt_expire_hours,
                )
                .map_err(AppError::internal)?;
                Ok(ApiResult::success(LoginResponse {
                    id: user.id.unwrap(),
                    username: user.username,
                    role: user.role,
                    token,
                }))
            } else {
                Ok(ApiResult::error("账号或密码错误"))
            }
        }
        Err(_) => Ok(ApiResult::error("账号或密码错误")),
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> Result<ApiResult<()>, AppError> {
    let username = user.username.trim();
    if username.is_empty() || user.password.is_empty() {
        return Ok(ApiResult::error("账号或密码不能为空"));
    }
    if username.len() > 50 {
        return Ok(ApiResult::error("用户名过长"));
    }
    if user.password.len() > 128 {
        return Ok(ApiResult::error("密码过长"));
    }
    if user.password.len() < 4 {
        return Ok(ApiResult::error("密码不能少于 4 位"));
    }
    let hashed = bcrypt::hash(&user.password, 10).map_err(|e| {
        tracing::error!("bcrypt hash failed: {e:?}");
        AppError::internal("服务器错误")
    })?;
    let conn = state.db.get()?;
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM user WHERE username = ?1",
            rusqlite::params![username],
            |row| row.get::<_, i64>(0),
        )
        .map(|c| c > 0)
        .unwrap_or(false);
    if exists {
        return Ok(ApiResult::error("用户名已存在"));
    }
    let total: i64 = conn
        .query_row("SELECT COUNT(*) FROM user", [], |row| row.get(0))
        .unwrap_or(0);
    if total > 0 {
        return Ok(ApiResult::error("注册已关闭，请联系管理员添加账号"));
    }
    let role = if total == 0 { "admin" } else { "user" };
    conn.execute(
        "INSERT INTO user (username, password, role) VALUES (?1, ?2, ?3)",
        rusqlite::params![username, hashed, role],
    )?;
    Ok(ApiResult::success_empty())
}

#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(req): Json<CreateUserRequest>,
) -> Result<ApiResult<()>, AppError> {
    let username = req.username.trim();
    if username.is_empty() || req.password.is_empty() {
        return Ok(ApiResult::error("账号或密码不能为空"));
    }
    if username.len() > 50 {
        return Ok(ApiResult::error("用户名过长"));
    }
    if req.password.len() < 4 {
        return Ok(ApiResult::error("密码不能少于 4 位"));
    }
    if req.password.len() > 128 {
        return Ok(ApiResult::error("密码过长"));
    }
    let hashed = bcrypt::hash(&req.password, 10).map_err(|_| AppError::internal("服务器错误"))?;
    let conn = state.db.get()?;
    let exists: bool = conn
        .query_row(
            "SELECT COUNT(*) FROM user WHERE username = ?1",
            rusqlite::params![username],
            |row| row.get::<_, i64>(0),
        )
        .map(|c| c > 0)
        .unwrap_or(false);
    if exists {
        return Ok(ApiResult::error("用户名已存在"));
    }
    conn.execute(
        "INSERT INTO user (username, password, role) VALUES (?1, ?2, 'user')",
        rusqlite::params![username, hashed],
    )?;
    Ok(ApiResult::success_empty())
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

#[derive(serde::Deserialize)]
pub struct DeleteUserRequest {
    pub id: i64,
}

pub async fn delete_user(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(req): Json<DeleteUserRequest>,
) -> Result<ApiResult<()>, AppError> {
    if req.id == _admin.0 {
        return Ok(ApiResult::error("不能删除自己"));
    }
    let conn = state.db.get()?;
    let role: String = conn.query_row(
        "SELECT role FROM user WHERE id = ?1",
        rusqlite::params![req.id],
        |row| row.get(0),
    )?;
    if role == "admin" {
        return Ok(ApiResult::error("不能删除其他管理员"));
    }
    conn.execute(
        "DELETE FROM bookmark WHERE user_id = ?1",
        rusqlite::params![req.id],
    )?;
    conn.execute(
        "DELETE FROM category WHERE user_id = ?1",
        rusqlite::params![req.id],
    )?;
    conn.execute("DELETE FROM user WHERE id = ?1", rusqlite::params![req.id])?;
    Ok(ApiResult::success_empty())
}

#[derive(serde::Serialize)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub role: String,
}

pub async fn list_users(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<ApiResult<Vec<UserInfo>>, AppError> {
    let conn = state.db.get()?;
    let mut stmt = conn.prepare("SELECT id, username, role FROM user ORDER BY id")?;
    let users: Vec<UserInfo> = stmt
        .query_map([], |row| {
            Ok(UserInfo {
                id: row.get(0)?,
                username: row.get(1)?,
                role: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ApiResult::success(users))
}
