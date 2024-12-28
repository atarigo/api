use crate::user::model::CreateUserDto;
use crate::user::model::UpdateUserDto;
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

pub async fn update_profile(
    service: web::Data<UserService>,
    path: web::Path<String>,
    user: web::Json<UpdateUserDto>,
) -> impl Responder {
    match service
        .update_user(&path.into_inner(), user.into_inner())
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
