use crate::user::service::debug;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use sqlx::SqlitePool;
use std::sync::Arc;

pub fn router(cfg: &mut web::ServiceConfig) -> () {
    cfg.service(
        web::scope("/user")
            .service(create_user)
            .service(retrieve_user)
            .service(update_user),
    );
}

#[post("/")]
async fn create_user(pool: web::Data<Arc<SqlitePool>>) -> impl Responder {
    // fixme: debug
    let _ = debug(&pool).await;
    HttpResponse::Ok().json("ok")
}

#[get("/")]
async fn retrieve_user() -> impl Responder {
    return format!("list user");
}

#[put("/{user_id}")]
async fn update_user(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("string: userId={}", path))
}
