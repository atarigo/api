mod user;

use crate::user::model::User;
use crate::user::repository::UserRepository;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use user::model::CreateUserDto;

const DB_URL: &str = "sqlite://sqlite.db";

async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

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

    let repository = UserRepository::new(pool.clone());
    let user = CreateUserDto {
        username: "aabar".to_string(),
        email: "aabar@example.com".to_string(),
    };

    match repository.create(user).await {
        Ok(user) => println!("{:?}", user),
        Err(e) => println!("insert error: {}", e.to_string()),
    };
    println!("__________");
    match repository.list().await {
        Ok(users) => {
            for user in users {
                println!("{:?}", user)
            }
        }
        Err(e) => println!("list error: {}", e.to_string()),
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // .wrap(middleware::Logger::default())
            .service(web::scope("/api").route("/ping", web::get().to(ping)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
