use anyhow::Result;
use sqlx::{Error as SqlxError, SqlitePool};

pub async fn create_role_table(pool: &SqlitePool) -> Result<(), SqlxError> {
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS roles (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
