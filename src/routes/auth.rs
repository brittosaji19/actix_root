use super::super::controllers;
use super::super::middlewares;
use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            //TEMPORARILY DISABLED FOR JWT IMPLEMENTATION
            .wrap(middlewares::authenticate::Authenticator)
            .route(
                "/login",
                web::get().to(|| HttpResponse::Ok().body("Welcome to Login route")),
            )
            .route("/signup", web::post().to(controllers::auth::signup))
            .route("/ack", web::get().to(controllers::auth::ack))
            .route("", web::get().to(controllers::auth::ack)),
    );
}
