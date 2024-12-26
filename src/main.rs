mod role;
mod user;

use crate::role::view::router as role_router;
use crate::user::view::router as user_router;
use actix_web::{web, App, HttpServer};

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
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
