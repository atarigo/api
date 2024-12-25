mod common;

use std::i64;

use crate::common::database::create_sqlite_pool;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: Option<i64>,
    username: String,
    email: String,
}

async fn insert_user(pool: &SqlitePool, user: &User) -> Result<i64> {
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

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let pool = create_sqlite_pool()
        .await
        .expect("Failed to create database pool");

    let user = User {
        id: None,
        username: "John Doe".to_string(),
        email: "john@example.com".to_string(),
    };

    match insert_user(&pool, &user).await {
        Ok(id) => println!("inserted user: {}", id),
        Err(e) => eprintln!("error inserting user: {}", e),
    };
    Ok(())
}
