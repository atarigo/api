use crate::user::model::CreateUserDto;
use crate::user::service::UserService;
use actix_web::{web, HttpResponse, Responder};

pub async fn register(
    service: web::Data<UserService>,
    user: web::Json<CreateUserDto>,
) -> impl Responder {
    match service.create_user(user.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn list_users(service: web::Data<UserService>) -> impl Responder {
    match service.list_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
