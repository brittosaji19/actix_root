use super::super::utils::{constants, encrypt, response_templates};
use actix_web::{HttpResponse, Responder};
extern crate jsonwebtoken;
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    id: String,
    exp: usize,
}

pub fn ack() -> impl Responder {
    HttpResponse::Ok().body("Welcome to auth controller")
}

//Basic signup function that should return a JSON response containing a success status and an access_token
//TODO full functionality not yet implemented
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
//Basic login function that should return a JSON response containing a success status and an access_token
//TODO full functionality not yet implemented
pub fn login() -> impl Responder {
    HttpResponse::Ok().body(format!("You have reached the login endpoint",))
}

//Creates a JSON Web Token byt taking a user id as a parameter and creating a User Struct with id value that was passed into the function
//The default expiry is set to be 1 hour
//Default headers are userd to encode however a reference to a JWTSECRET is supplied as key for token generation
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

// Decodes the incoming token &str to generate the User Struct that was encoded into the token
// A leeway of 60 seconds is added to adjust for any possible clock skew while checking for token expiry
//The JWTSECRET used while generating the token is supplied as key to accurately decrypt the token
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
    token_data
}
