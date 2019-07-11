use crate::ConnectionPool;
use database::*;
use dotenv::dotenv;
use std::{env, io::stdin};

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_pool = ConnectionPool::new(&database_url);

    let mut cat_name = String::new();

    println!("Enter new category name");
    stdin().read_line(&mut cat_name).unwrap();
    let cat_name = cat_name.trim_end(); // Remove the trailing newline

    let category = connection_pool.create_category(&cat_name);
    println!(
        "\nSaved new category with name {} and id {}",
        cat_name, category.id
    );
}
