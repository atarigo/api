use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{Error as SqlxError, SqlitePool};
use std::i64;

pub async fn create_table(pool: &SqlitePool) -> Result<(), SqlxError> {
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT NOT NULL UNIQUE,
                username TEXT NOT NULL UNIQUE,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

/*
pub async fn create_user(pool: &SqlitePool, user: &User) -> Result<i64> {
    let result = sqlx::query(
        r#"
        INSERT INTO users (username, email)
        VALUES (?, ?)
        "#,
    )
    .bind(&user.username)
    .bind(&user.email)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
*/

// pub async fn read_user() {}

// update user

// delete user
