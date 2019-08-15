use super::super::utils::{constants, encrypt, response_templates};
use actix_web::{HttpResponse, Responder};
extern crate jsonwebtoken;
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    exp: usize,
}

pub fn ack() -> impl Responder {
    HttpResponse::Ok().body("Hello from auth controller")
}
pub fn signup() -> impl Responder {
    //TEMPORARY IMPLEMENTATION FOR TESTING
    let hash_response = encrypt::generate_hash("hello".to_string());
    let hash_verify = encrypt::verify_hash("hello".to_string(), &hash_response);
    create_and_register_token("cat");
    HttpResponse::Ok().body(format!(
        "You have reached the signup endpoint and your key is {} verification  {}",
        hash_response, hash_verify
    ))
}

pub fn create_and_register_token(id: &str) -> String {
    let expiry = std::time::SystemTime::now()
        .checked_add(std::time::Duration::new(3600, 0))
        .unwrap()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let current_user = User {
        id: id.to_owned(),
        exp: expiry as usize,
    };
    let jwt_secret = constants::JWT_SECRET;
    let key: &[u8] = jwt_secret.as_ref();
    let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &current_user, key);
    println!("{:?}", token);
    token.unwrap()
}

pub fn decode_token(
    token: &str,
) -> Result<jsonwebtoken::TokenData<User>, jsonwebtoken::errors::Error> {
    let jwt_secret = constants::JWT_SECRET;
    let key: &[u8] = jwt_secret.as_ref();
    let token_data = jsonwebtoken::decode::<User>(
        &token,
        key,
        &jsonwebtoken::Validation {
            leeway: 60,
            ..Default::default()
        },
    );
    // let token_data=jsonwebtoken::dangerous_unsafe_decode::<User>(&token);
    token_data
}
