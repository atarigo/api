use crate::user::model::{CreateUserDto, User};
use sqlx::SqlitePool;
use uuid::Uuid;

use super::model::UpdateUserDto;

#[derive(Clone)]
pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create(&self, user: CreateUserDto) -> Result<User, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let user = sqlx::query_as::<_, User>(
            r#"
                INSERT INTO users (id, email, username) 
                VALUES (?, ?, ?)
                RETURNING *
            "#,
        )
        .bind(Uuid::new_v4().to_string())
        .bind(user.email)
        .bind(user.username)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(user)
    }

    pub async fn update(&self, id: &str, user: UpdateUserDto) -> Result<User, sqlx::Error> {
        let mut fields = Vec::new();
        let mut values = Vec::new();

        if user.email.is_some() {
            fields.push("email = ?");
            values.push(user.email.as_ref().unwrap().as_str());
        }

        if user.username.is_some() {
            fields.push("username = ?");
            values.push(user.username.as_ref().unwrap().as_str());
        }

        let statement = format!(
            "UPDATE users SET {} WHERE id = ? RETRUNING *",
            fields.join(", ")
        );

        let mut tx = self.pool.begin().await?;
        let mut query = sqlx::query_as::<_, User>(&statement);
        for value in values {
            query = query.bind(value)
        }
        let updated_user = query.bind(id).fetch_one(&mut *tx).await?;

        tx.commit().await?;
        Ok(updated_user)
    }
}
