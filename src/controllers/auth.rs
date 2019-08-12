use actix_web::{HttpResponse, Responder};
pub fn ack() -> impl Responder {
    HttpResponse::Ok().body("Hello from auth controller")
}
