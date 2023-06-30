use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
// use schema::users;
// use diesel::result;
use self::models::{NewUser, User};
pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut SqliteConnection, username_in: &str, email_in: &str, password_in: &str) -> User {
    use crate::schema::users::dsl::*;

    let new_user = NewUser { username: username_in, email: email_in, password: password_in };

    diesel::insert_into(users).values(&new_user).execute(conn)
        .expect("Error saving new user");

    users.order(id.desc()).first(conn)
        .expect("Error retrieving new user")
}
