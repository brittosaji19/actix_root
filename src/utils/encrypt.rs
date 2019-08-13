extern crate bcrypt;
use bcrypt::{hash, verify, DEFAULT_COST};
pub fn generate_hash(data: String) -> String {
    let hashed_data = hash(data, 4);
    let unwrapped_result = hashed_data.unwrap();
    return unwrapped_result;
}

pub fn verify_hash(password: String, hash_string: &str) -> bool {
    let hash_verified = verify(password, hash_string);
    return hash_verified.unwrap();
}
