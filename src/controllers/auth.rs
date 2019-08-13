use super::super::utils::{encrypt, response_templates};
use actix_web::{HttpResponse, Responder};
pub fn ack() -> impl Responder {
    HttpResponse::Ok().body("Hello from auth controller")
}
pub fn signup() -> impl Responder {
    //TEMPORARY IMPLEMENTATION FOR TESTING
    let hash_response = encrypt::generate_hash("hello".to_string());
    let hash_verify = encrypt::verify_hash("hello".to_string(), &hash_response);
    response_templates::unauthorized_request()
    // HttpResponse::Ok().body(format!(
    //     "You have reached the signup endpoint and your key is {} verification  {}",
    //     hash_response, hash_verify
    // ))
}
