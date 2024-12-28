use crate::user::model::{CreateUserDto, UpdateUserDto, User};
use sqlx::SqlitePool;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("Record not found")]
    NotFound,

    #[error("Duplicate key: {0}")]
    UniqueViolation(&'static str),

    #[error("Invalid field: {0}")]
    InvalidField(&'static str),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[derive(Clone)]
pub struct UserRepository {
    pool: SqlitePool,
}

impl UserRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    fn map_database_error(err: sqlx::Error) -> RepositoryError {
        match &err {
            sqlx::Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                    RepositoryError::UniqueViolation("Duplicate key violation")
                } else {
                    // todo: a workaround
                    let error_msg = db_err.message();
                    if error_msg.contains("NOT NULL") {
                        RepositoryError::InvalidField("Required field is missing")
                    } else {
                        RepositoryError::DatabaseError(err)
                    }
                }
            }
            sqlx::Error::RowNotFound => RepositoryError::NotFound,
            _ => RepositoryError::DatabaseError(err),
        }
    }

    pub async fn list(&self) -> Result<Vec<User>, RepositoryError> {
        sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
            .map_err(Self::map_database_error)
    }

    /*
    pub async fn get(&self, id: &str) -> Result<Option<User>, RepositoryError> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(Self::map_database_error)
    }
    */

    pub async fn create(&self, user: CreateUserDto) -> Result<User, RepositoryError> {
        let mut tx = self.pool.begin().await?;

        let result = sqlx::query_as::<_, User>(
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
        .await
        .map_err(Self::map_database_error)?;

        tx.commit().await?;
        Ok(result)
    }

    pub async fn update(&self, id: &str, user: UpdateUserDto) -> Result<User, RepositoryError> {
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
            r#"
                UPDATE users 
                SET {} 
                WHERE id = ? 
                RETRUNING *
            "#,
            fields.join(", ")
        );

        let mut tx = self.pool.begin().await?;
        let mut query = sqlx::query_as::<_, User>(&statement);
        for value in values {
            query = query.bind(value)
        }
        let result = query
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(Self::map_database_error)?;
        tx.commit().await?;

        match result {
            Some(user) => Ok(user),
            None => Err(RepositoryError::NotFound),
        }
    }

    pub async fn delete(&self, id: &str) -> Result<bool, RepositoryError> {
        let mut tx = self.pool.begin().await?;
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(Self::map_database_error)?;
        tx.commit().await?;

        match result.rows_affected() {
            0 => Err(RepositoryError::NotFound),
            _ => Ok(true),
        }
    }
}
