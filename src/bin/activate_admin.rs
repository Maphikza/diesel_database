// use self::models::User;
use diesel::prelude::*;
use diesel_user_db::*;
use std::env::args;

fn main() {
    use self::schema::users::dsl::{users, admin};

    let id = args()
        .nth(1)
        .expect("activate_admin requires a user id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let user = diesel::update(users.find(id))
        .set(admin.eq(true)).execute(connection);

    println!("User {:?} activated as admin.", user.unwrap())
}