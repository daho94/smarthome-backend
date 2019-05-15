use database::*;
use djangohashers::{make_password_with_algorithm, Algorithm::BCryptSHA256};
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();

    let mut username = String::new();
    let mut password = String::new();

    println!("Enter youe username!");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end(); // Remove the trailing newline

    println!("Enter a password for {}", username);
    stdin().read_line(&mut password).unwrap();
    let password = password.trim_end();
    let user = create_user(
        &connection,
        username,
        &make_password_with_algorithm(password, BCryptSHA256),
    );
    println!("\nSaved user {} with id {}", username, user.id);
}
