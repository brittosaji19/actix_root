use super::super::schema::users;
use super::PgConnection;
use crate::diesel::RunQueryDsl;
use crate::diesel::{Insertable, Queryable};
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
