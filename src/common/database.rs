use anyhow::Result;
use sqlx::sqlite::SqlitePool;

pub async fn create_sqlite_pool() -> Result<SqlitePool> {
    let _pool = SqlitePool::connect("sqlite::memory:").await?;

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL UNIQUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
            "#,
    )
    .execute(&_pool)
    .await?;

    Ok(_pool)
}
