use diesel::prelude::*;
use diesel_user_db::*;
use std::{env::args};

fn main() {
    use self::schema::users::dsl::*;

    let target = args()
        .nth(1)
        .expect("expected a target to match against.");

    let user_id: i32 = target.parse().expect("Expected a user id which is should be i32 integer.");

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(users.filter(id.eq(user_id)))
        .execute(connection)
        .expect("Error deleting user.");

    println!("Deleted users: {} and the deleted user id is, {}.", num_deleted, user_id);
}