use crate::user::model::{CreateUserDto, User};
use sqlx::SqlitePool;
use uuid::Uuid;

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
}
