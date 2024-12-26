use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{Error as SqlxError, FromRow, SqlitePool};

pub async fn create_table(pool: &SqlitePool) -> Result<(), SqlxError> {
    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                username TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub id: String,
    pub email: String,
    pub username: String,
}

pub async fn create_user(pool: &SqlitePool, user: &CreateUser) -> Result<String> {
    sqlx::query(
        r#"
            INSERT INTO users (id, email, username)
            VALUES (?, ?, ?)
        "#,
    )
    .bind(&user.id)
    .bind(&user.email)
    .bind(&user.username)
    .execute(pool)
    .await?;

    Ok(user.id.clone())
}

#[derive(FromRow, Serialize)]
pub struct ReadUser {
    pub id: String,
    pub email: String,
    pub username: String,
    pub created_at: i64,
}

pub async fn read_user(pool: &SqlitePool, id: Option<&str>) -> Result<Vec<ReadUser>> {
    let query = match id {
        Some(id) => sqlx::query_as::<_, ReadUser>(
            r#"
                SELECT id, email, username, created_at
                FROM users
                WHERE id = ?
            "#,
        )
        .bind(id),
        None => sqlx::query_as::<_, ReadUser>(
            r#"
                SELECT id, email, username, created_at
                FROM users
            "#,
        ),
    };

    let users = query.fetch_all(pool).await?;
    Ok(users)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
}

pub async fn update_user(pool: &SqlitePool, id: &str, user: &UpdateUser) -> Result<()> {
    if user.email.is_none() && user.username.is_none() {
        return Err(anyhow::anyhow!(
            "At least one field must be provided for update"
        ));
    }

    let mut query = String::from("UPDATE users SET");
    let mut params = Vec::new();
    let mut first = true;

    if let Some(email) = &user.email {
        query.push_str(" email = ?");
        params.push(email);
        first = false;
    }

    if let Some(username) = &user.username {
        if !first {
            query.push_str(",");
        }
        query.push_str(" username = ?");
        params.push(username);
    }

    query.push_str(" WHERE id = ?");

    let mut db_query = sqlx::query(&query);

    // Bind all parameters in order
    for param in &params {
        db_query = db_query.bind(param);
    }
    db_query = db_query.bind(id);

    db_query.execute(pool).await?;

    Ok(())
}

pub async fn delete_user(pool: &SqlitePool, id: &str) -> Result<()> {
    let result = sqlx::query(
        r#"
            DELETE FROM users
            WHERE id = ?
        "#,
    )
    .bind(id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(anyhow::anyhow!("User not found"));
    }
    Ok(())
}
