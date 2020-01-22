use crate::ConnectionPool;
use database::*;
use dotenv::dotenv;
use serde_json::Value;
use std::{env, io::stdin};

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_pool = ConnectionPool::new(&database_url);

    let mut username = String::new();
    let mut dashboard_name = String::new();
    let mut is_default = String::new();
    let mut icon = String::new();
    let mut dashboard_folder_id = String::new();

    println!("Enter username");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end();

    println!("Enter dashboard name");
    stdin().read_line(&mut dashboard_name).unwrap();
    let dashboard_name = dashboard_name.trim_end();

    println!("Enter icon name");
    stdin().read_line(&mut icon).unwrap();
    let icon = icon.trim_end();

    println!("Enter dashboard folder id");
    stdin().read_line(&mut dashboard_folder_id).unwrap();
    let dashboard_folder_id: i32 = dashboard_folder_id.trim_end().parse().unwrap();

    println!("Should this dashboard be your default dashboard? (y/n)");
    stdin().read_line(&mut is_default).unwrap();
    let is_default = match is_default.trim_end() {
        "y" | "Y" => true,
        "n" | "N" => false,
        _ => false,
    };

    if let Ok(user) = connection_pool.get_user(&username) {
        let settings = r#"[]"#;

        // Parse the string of data into serde_json::Value.
        let settings: Value = serde_json::from_str(settings).expect("Failed to parse JSON input");
        let dashboard = connection_pool.create_dashboard_for_user(
            &user,
            &dashboard_name,
            &settings,
            is_default,
            &icon,
            dashboard_folder_id,
        );
        println!(
            "Saved new dashboard with name {} and id {}",
            dashboard.name, dashboard.id
        );
    } else {
        println!("User with name '{}' does not exist", username);
    }
}
