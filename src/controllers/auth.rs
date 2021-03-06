use super::super::models;
use super::super::utils::{constants, encrypt, response_templates};
use actix_web::{web, HttpResponse, Responder};
extern crate jsonwebtoken;
extern crate serde_json;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserToken {
    id: String,
    exp: usize,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Credentials {
    email: String,
    password: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct SessionInfo {
    user: models::user::User,
    access_token: String,
}
pub fn ack() -> impl Responder {
    HttpResponse::Ok().body("Welcome to auth controller")
}

//Basic signup function that should return a JSON response containing a success status and an access_token
//TODO full functionality not yet implemented
pub fn signup(cred: web::Json<models::user::NewUser>) -> impl Responder {
    //TEMPORARY IMPLEMENTATION FOR TESTING
    let hash_response = encrypt::generate_hash(cred.password.to_string());
    let _hash_verify = encrypt::verify_hash("hello".to_string(), &hash_response);
    create_and_register_token("cat");
    let user = models::user::create_user(
        &models::establish_connection(),
        cred.name.to_owned(),
        cred.email.to_owned(),
        hash_response.to_owned(),
    );
    let response = SessionInfo {
        user: user.clone(),
        access_token: create_and_register_token(user.id.to_string().as_ref()),
    };
    println!("{:?}", response);
    response_templates::data(serde_json::to_value(&response).unwrap())
}
//Basic login function that should return a JSON response containing a success status and an access_token
//TODO full functionality not yet implemented
pub fn login(cred: web::Json<Credentials>) -> impl Responder {
    // HttpResponse::Ok().body(format!(
    //     "You have reached the login endpoint email : {} password: : {} hash : {}",
    //     cred.email, cred.password,encrypt::generate_hash(cred.password.to_owned())
    // ))
    let cred_t: Credentials = cred.into_inner();
    let db_response =
        models::user::get_user(&models::establish_connection(), cred_t.email.to_owned());
    if db_response.is_err() {
        return response_templates::error(400, "Invalid Email".to_string());
    }
    println!("User :: {:?}", db_response);
    let fetched_user = db_response.unwrap();
    let verify_hash =
        encrypt::verify_hash(cred_t.password.to_owned(), fetched_user.password.as_ref());
    println!("{:?}", verify_hash);
    response_templates::data(serde_json::to_value(cred_t).unwrap())
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
    let current_user = UserToken {
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
) -> Result<jsonwebtoken::TokenData<UserToken>, jsonwebtoken::errors::Error> {
    let jwt_secret = constants::JWT_SECRET;
    let key: &[u8] = jwt_secret.as_ref();
    let token_data = jsonwebtoken::decode::<UserToken>(
        &token,
        key,
        &jsonwebtoken::Validation {
            leeway: 60,
            ..Default::default()
        },
    );
    token_data
}
