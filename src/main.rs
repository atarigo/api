mod database;
mod settings;
mod user;

use crate::database::Database;
use crate::settings::Settings;
use crate::user::controller;
use crate::user::repository::UserRepository;
use crate::user::service::UserService;
use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()))
        .init();

    info!("Starting server...");

    let settings = match Settings::new() {
        Ok(settings) => settings,
        Err(e) => {
            println!("{}", e.to_string());
            std::process::exit(1);
        }
    };

    let pool = match Database::new(settings.db_addr()).await {
        Ok(db) => db.get_pool(),
        Err(e) => {
            println!("{}", e.to_string());
            std::process::exit(1);
        }
    };

    let repository = UserRepository::new(pool.clone());
    let service = UserService::new(repository);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://atarigo.io")
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .max_age(3600)
            .supports_credentials();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(service.clone()))
            .service(
                web::scope("/api")
                    .route("/user", web::get().to(controller::list_users))
                    .route("/user", web::post().to(controller::register))
                    .route("/user", web::put().to(controller::update_profile))
                    .route("/user", web::delete().to(controller::delete_account)),
            )
    })
    .bind(settings.server_addr())?
    .run()
    .await
}
