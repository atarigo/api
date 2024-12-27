mod role;
mod user;

use crate::role::view::router as role_router;
use crate::user::view::router as user_router;
use actix_web::{web, App, HttpServer};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create Sqlite Database
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    // Connect to sqlite
    let pool = SqlitePool::connect(DB_URL).await.unwrap();

    // Create tables
    // ! todo: move this sections to migrations
    let result = sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS users (
                id TEXT PRIMARY KEY,
                email TEXT NOT NULL UNIQUE,
                username TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
        "#,
    )
    .execute(&pool)
    .await
    .unwrap();
    println!("Create user table result: {:?}", result);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .configure(user_router)
                    .configure(role_router),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
