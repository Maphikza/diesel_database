use self::models::*;
use diesel::prelude::*;
use diesel_user_db::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .filter(admin.eq(true))
        .limit(2)
        .select(User::as_select())
        .load(connection)
        .expect("There was an error whenn loading users");

    println!("Displaying {} users\n", results.len());
    for user in results {
        println!("User: {}", user.username);
        println!("\n----------\n");
        println!("With email, {}", user.email);
    }
}
