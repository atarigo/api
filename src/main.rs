mod settings;
mod user;

use crate::settings::Settings;
use crate::user::controller;
use crate::user::repository::UserRepository;
use crate::user::service::UserService;
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new().expect("invalid settings");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Create Sqlite Database
    let db_url = settings.db_addr();
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    // Connect to sqlite
    let pool = SqlitePool::connect(settings.db_addr()).await.unwrap();

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
            .app_data(web::Data::new(service.clone()))
            .service(
                web::scope("/api")
                    .route("/user", web::get().to(controller::list_users))
                    .route("/user", web::post().to(controller::register))
                    .route("/user", web::put().to(controller::update_profile)),
            )
    })
    .bind(settings.server_addr())?
    .run()
    .await
}
