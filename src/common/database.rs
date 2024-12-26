use crate::role::model::create_role_table;
use crate::user::model::create_user_table;
use sqlx::sqlite::SqlitePool;
use sqlx::Error as SqlxError;

#[allow(dead_code)]
#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(SqlxError),
    MigrationError(SqlxError),
}

pub async fn create_sqlite_pool() -> Result<SqlitePool, DatabaseError> {
    let pool = SqlitePool::connect("sqlite::memory:")
        .await
        .map_err(DatabaseError::ConnectionError)?;

    migrate(&pool)
        .await
        .map_err(DatabaseError::MigrationError)?;

    Ok(pool)
}

pub async fn migrate(pool: &SqlitePool) -> Result<(), SqlxError> {
    create_user_table(pool).await?;
    create_role_table(pool).await?;

    Ok(())
}
