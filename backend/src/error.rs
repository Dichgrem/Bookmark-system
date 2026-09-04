use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl AppError {
    pub fn internal(msg: impl Into<String>) -> Self {
        Self {
            message: msg.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("request failed: {}", self.message);
        let body = Json(json!({
            "code": 500,
            "msg": self.message,
            "data": null
        }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        if let rusqlite::Error::SqliteFailure(ffi, detail) = &e {
            tracing::error!(
                "database error: code={:?} extended_code={} detail={:?}",
                ffi.code,
                ffi.extended_code,
                detail
            );
        } else {
            tracing::error!("database error: {e:?}");
        }
        Self {
            message: format!("数据库错误: {}", e),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        Self {
            message: format!("序列化错误: {}", e),
        }
    }
}

impl From<std::sync::PoisonError<std::sync::MutexGuard<'_, rusqlite::Connection>>> for AppError {
    fn from(e: std::sync::PoisonError<std::sync::MutexGuard<'_, rusqlite::Connection>>) -> Self {
        Self {
            message: format!("数据库锁错误: {}", e),
        }
    }
}

impl From<r2d2::Error> for AppError {
    fn from(e: r2d2::Error) -> Self {
        Self {
            message: format!("数据库连接错误: {}", e),
        }
    }
}

impl From<tokio::task::JoinError> for AppError {
    fn from(e: tokio::task::JoinError) -> Self {
        Self {
            message: format!("内部服务错误: {}", e),
        }
    }
}

impl From<String> for AppError {
    fn from(e: String) -> Self {
        Self { message: e }
    }
}
