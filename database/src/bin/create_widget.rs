use crate::ConnectionPool;
use database::*;
use dotenv::dotenv;
use std::{env, io::stdin};

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_pool = ConnectionPool::new(&database_url);

    let mut widget_name = String::new();
    let mut component_key = String::new();
    let mut category_name = String::new();
    let mut icon_name = String::new();

    println!("Enter new widget name");
    stdin().read_line(&mut widget_name).unwrap();
    let widget_name = widget_name.trim_end();

    println!("Enter component_key");
    stdin().read_line(&mut component_key).unwrap();
    let component_key = component_key.trim_end();

    println!("Enter category");
    stdin().read_line(&mut category_name).unwrap();
    let category_name = category_name.trim_end();

    println!("Enter icon");
    stdin().read_line(&mut icon_name).unwrap();
    let icon_name = icon_name.trim_end();

    if let Ok(category) = connection_pool.get_category(&category_name) {
        let widget = connection_pool.create_widget_for_category(
            &widget_name,
            &component_key,
            &category,
            &icon_name,
        );
        println!(
            "\nSaved new widget with name {} and id {}",
            widget_name, widget.id
        );
    } else {
        println!("Category does not exist!");
    }
}
