mod database;
mod settings;
mod user;

use crate::database::Database;
use crate::settings::Settings;
use crate::user::controller;
use crate::user::repository::UserRepository;
use crate::user::service::UserService;
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

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
