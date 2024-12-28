use crate::user::model::CreateUserDto;
use crate::user::model::UpdateUserDto;
use crate::user::service::{UserService, UserServiceError};
use actix_web::{web, HttpResponse, Responder};

fn map_errors(err: UserServiceError) -> HttpResponse {
    match err {
        UserServiceError::NotFound => HttpResponse::NotFound().body("User not found"),
        UserServiceError::ConflictError(msg) => HttpResponse::Conflict().body(msg),
        UserServiceError::ValidationError(msg) => HttpResponse::BadRequest().body(msg),
        UserServiceError::DatabaseError(_) => {
            HttpResponse::InternalServerError().body("Database error")
        }
    }
}

pub async fn register(
    service: web::Data<UserService>,
    user: web::Json<CreateUserDto>,
) -> impl Responder {
    match service.create_user(user.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => map_errors(e),
    }
}

pub async fn list_users(service: web::Data<UserService>) -> impl Responder {
    match service.list_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => map_errors(e),
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
        Err(e) => map_errors(e),
    }
}

pub async fn delete_account(
    service: web::Data<UserService>,
    path: web::Path<String>,
) -> impl Responder {
    match service.delete_user(&path.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().body(""),
        Err(e) => map_errors(e),
    }
}
