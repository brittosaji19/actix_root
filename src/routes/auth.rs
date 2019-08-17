use super::super::controllers;
use super::super::middlewares;
use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(controllers::auth::login))
            .route("/signup", web::post().to(controllers::auth::signup))
            .route("/", web::get().to(controllers::auth::ack))
            .route("", web::get().to(controllers::auth::ack)),
    );
}
