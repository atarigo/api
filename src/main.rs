mod user;

use crate::user::controller;
use crate::user::repository::UserRepository;
use crate::user::service::UserService;
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://sqlite.db";

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

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

    let repository = UserRepository::new(pool.clone());
    let service = UserService::new(repository);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(service.clone())
            .service(
                web::scope("/api")
                    .route("/user", web::get().to(controller::list_users))
                    .route("/user", web::post().to(controller::register)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
