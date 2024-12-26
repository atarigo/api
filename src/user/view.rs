// use crate::user::model::NewUser;
// use crate::user::service::add_user;
use actix_web::{get, post, put, web, HttpResponse, Responder};
use serde::Serialize;

pub fn router(cfg: &mut web::ServiceConfig) -> () {
    cfg.service(
        web::scope("/user")
            .service(create_user)
            .service(retrieve_user)
            .service(update_user),
    );
}

#[derive(Serialize)]
struct CreatedUser {
    id: i32,
}

#[post("/")]
async fn create_user() -> impl Responder {
    let user = CreatedUser { id: 30 };

    HttpResponse::Ok().json(user)
}

#[get("/")]
async fn retrieve_user() -> impl Responder {
    return format!("list user");
}

#[put("/{user_id}")]
async fn update_user(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("string: userId={}", path))
}
