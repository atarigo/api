mod common;
mod role;
mod user;

use crate::common::database::create_sqlite_pool;
use crate::role::view::router as role_router;
use crate::user::view::router as user_router;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = match create_sqlite_pool().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    };

    let pool = Arc::new(pool);

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
