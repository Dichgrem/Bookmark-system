use std::env;

#[derive(Clone)]
pub struct Config {
    pub jwt_expire_hours: usize,
    pub database_path: String,
    pub port: u16,
    pub cors_origin: String,
    pub frontend_dir: String,
    pub allow_private_urls: bool,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            jwt_expire_hours: env::var("JWT_EXPIRE_HOURS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(72),
            database_path: env::var("DATABASE_PATH")
                .unwrap_or_else(|_| ".local/bookmark.db".into()),
            port: env::var("PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8989),
            cors_origin: env::var("CORS_ORIGIN").unwrap_or_else(|_| "http://localhost:5173".into()),
            frontend_dir: env::var("FRONTEND_DIR").unwrap_or_else(|_| "../frontend/dist".into()),
            allow_private_urls: env::var("ALLOW_PRIVATE_URLS")
                .ok()
                .map(|s| s == "true" || s == "1")
                .unwrap_or_else(|| cfg!(debug_assertions)),
        }
    }
}
