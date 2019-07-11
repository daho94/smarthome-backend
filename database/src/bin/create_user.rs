use crate::ConnectionPool;
use database::*;
// use djangohashers::{make_password_with_algorithm, Algorithm::BCryptSHA256};
use bcrypt::{DEFAULT_COST, hash};
use dotenv::dotenv;
use std::{env, io::stdin};

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_pool = ConnectionPool::new(&database_url);

    let mut username = String::new();
    let mut password = String::new();

    println!("Enter your username!");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end(); // Remove the trailing newline

    println!("Enter a password for {}", username);
    stdin().read_line(&mut password).unwrap();
    let password = password.trim_end();
    let user = connection_pool.create_user(
        &username,
        &hash(password,  DEFAULT_COST).expect("Couldn't hash passord"),
    );
    println!("\nSaved user {} with id {}", username, user.id);
}
