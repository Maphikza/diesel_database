use diesel_user_db::*;
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut username = String::new();
    let mut email = String::new();
    let mut password = String::new();
    
    println!("What would you like your username to be?");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end();

    println!("Ok! What's your email {}, (Press {} when finished)\n", username, EOF);

    stdin().read_to_string(&mut email).unwrap();


    println!("Excellent! What would you like for your password to be?");
    stdin().read_line(&mut password).unwrap();
    let password = password.trim_end();

    let user = create_user(connection, username, &email, password);
    println!("\nSaved user {} with email {}", user.username, user.email);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";