use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

pub fn router(cfg: &mut web::ServiceConfig) -> () {
    cfg.service(
        web::scope("/role")
            .service(create_role)
            .service(retrieve_role)
            .service(update_role)
            .service(delete_role),
    );
}

#[post("/")]
async fn create_role() -> impl Responder {
    HttpResponse::Ok().body("create a role")
}

#[get("/")]
async fn retrieve_role() -> impl Responder {
    HttpResponse::Ok().body("read roles")
}

#[put("/{role_id}")]
async fn update_role(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("update a role, {}", path))
}

#[delete("/{role_id}")]
async fn delete_role(path: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("delete a role, {}", path))
}
