use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
use tracing::{error, info};

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        Self::create_database_if_not_exists(database_url).await?;

        let pool = SqlitePool::connect(database_url).await?;
        info!("Connection to database successful");

        Self::run_migrations(&pool).await?;
        info!("Migrate successfully");

        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> Pool<Sqlite> {
        self.pool.clone()
    }

    async fn create_database_if_not_exists(database_url: &str) -> Result<(), sqlx::Error> {
        match Sqlite::database_exists(database_url).await {
            Ok(exists) => match exists {
                true => Ok(()),
                false => match Sqlite::create_database(database_url).await {
                    Ok(_) => {
                        info!("create database success");
                        Ok(())
                    }
                    Err(e) => {
                        error!("failed to create database, {}", database_url);
                        Err(e)
                    }
                },
            },
            Err(e) => {
                error!("failed to check database status, {}", e.to_string());
                Err(e)
            }
        }
    }

    async fn run_migrations(pool: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                username TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            "#,
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
