use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        Self::create_database_if_not_exists(database_url).await?;

        let pool = SqlitePool::connect(database_url).await?;

        Self::run_migrations(&pool).await?;

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
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                },
            },
            Err(e) => Err(e),
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
