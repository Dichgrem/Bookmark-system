use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use rusqlite_migration::{Migrations, M};
use std::path::Path;

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn init_db(db_path: &str) -> (DbPool, String) {
    let path = Path::new(db_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("Failed to create database directory");
    }

    let mut conn = Connection::open(path).expect("Failed to open SQLite database");

    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
        .expect("Failed to set pragmas");

    let migrations = Migrations::new(vec![
        M::up(
            "CREATE TABLE IF NOT EXISTS secrets (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL,
                role TEXT NOT NULL DEFAULT 'user'
            );
            CREATE TABLE IF NOT EXISTS category (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                user_id INTEGER NOT NULL,
                parent_id INTEGER,
                sort_order INTEGER DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS bookmark (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                url TEXT NOT NULL,
                icon TEXT,
                category_id INTEGER,
                user_id INTEGER NOT NULL,
                sort_order INTEGER DEFAULT 0
            );",
        ),
        M::up(
            "CREATE INDEX IF NOT EXISTS idx_bookmark_user_id ON bookmark(user_id);
             CREATE INDEX IF NOT EXISTS idx_bookmark_category_id ON bookmark(category_id);
             CREATE INDEX IF NOT EXISTS idx_bookmark_url_user ON bookmark(url, user_id);
             CREATE INDEX IF NOT EXISTS idx_category_user_id ON category(user_id);
             CREATE INDEX IF NOT EXISTS idx_category_parent_id ON category(parent_id);",
        ),
    ]);

    migrations
        .to_latest(&mut conn)
        .expect("Failed to run database migrations");

    let existing: Option<String> = conn
        .query_row(
            "SELECT value FROM secrets WHERE key = 'jwt_secret'",
            [],
            |row| row.get(0),
        )
        .ok();

    let secret = std::env::var("JWT_SECRET").ok().unwrap_or_else(|| {
        existing.unwrap_or_else(|| {
            let bytes: [u8; 32] = rand::random();
            let hex = hex::encode(bytes);
            conn.execute(
                "INSERT INTO secrets (key, value) VALUES ('jwt_secret', ?1)",
                rusqlite::params![hex],
            )
            .expect("Failed to persist jwt_secret");
            hex
        })
    });

    drop(conn);

    let manager = SqliteConnectionManager::file(path);
    let pool = Pool::builder()
        .max_size(4)
        .build(manager)
        .expect("Failed to create database pool");

    (pool, secret)
}

pub fn ensure_admin_user(pool: &DbPool, username: &str, password: &str) {
    let conn = pool.get().expect("Failed to get database connection");
    let total: i64 = conn
        .query_row("SELECT COUNT(*) FROM user", [], |row| row.get(0))
        .expect("Failed to query user count");
    if total > 0 {
        return;
    }
    let hashed = bcrypt::hash(password, 10).expect("Failed to hash admin password");
    conn.execute(
        "INSERT INTO user (username, password, role) VALUES (?1, ?2, 'admin')",
        rusqlite::params![username, hashed],
    )
    .expect("Failed to create admin user");
}
