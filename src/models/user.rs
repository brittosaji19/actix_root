use super::super::schema::users;
use super::PgConnection;
use crate::diesel::RunQueryDsl;
use crate::diesel::{expression_methods::*, query_dsl::*, result::Error, Insertable, Queryable,QueryResult,OptionalExtension};
use uuid::Uuid;
#[derive(Queryable, Deserialize, Serialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

pub fn create_user(conn: &PgConnection, name: String, email: String, password: String) -> User {
    let newuser = NewUser {
        name: name,
        email: email,
        password: password,
    };

    diesel::insert_into(users::table)
        .values(&newuser)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_user(conn: &PgConnection, user_email: String) -> Result<User, String> {
    use super::super::schema::users::dsl::*;
    let result:Option<User> = users
        .filter(email.eq(user_email))
        .first::<User>(conn).optional()
        .expect("Invalid Email Provided");
    if result.is_some(){
        return Ok(result.unwrap())
    }else{
        return Err("Invalid Email Id".to_string())
    }
}
