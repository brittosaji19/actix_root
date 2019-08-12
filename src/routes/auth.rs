use super::super::controllers;
use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route(
                "/login",
                web::get().to(|| HttpResponse::Ok().body("Welcome to Login route")),
            )
            .route("/ack", web::get().to(controllers::auth::ack))
            .route("", web::get().to(controllers::auth::ack)),
    );
}
