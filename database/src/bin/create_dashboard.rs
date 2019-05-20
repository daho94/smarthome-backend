use crate::models::user::User;
use crate::ConnectionPool;
use database::*;
use dotenv::dotenv;
use serde_json::Value;
use std::env;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_pool = ConnectionPool::new(&database_url);

    let user = User {
        id: 6,
        username: "top".into(),
        password: "IDC".into(),
    };

    let settings = r#"
        [
            {
                "x": 0,
                "y": 0,
                "w": 2,
                "h": 2,
                "i": "0a997649-7af4-44e2-bdcd-4a100353b735",
                "c": "SocketWidget",
                "settings": {
                "title": {
                    "val": "CPU Temp",
                    "type": "input"
                },
                "showTitle": {
                    "val": true,
                    "type": "checkbox"
                }
                },
                "moved": false
            },
            {
                "x": 2,
                "y": 0,
                "w": 2,
                "h": 4,
                "i": "ab9df151-6070-465a-bf79-92698150c705",
                "c": "SocketWidget",
                "settings": {
                "title": {
                    "val": "CPU Temp",
                    "type": "input"
                },
                "showTitle": {
                    "val": true,
                    "type": "checkbox"
                }
                },
                "moved": false
            },
            {
                "x": 4,
                "y": 0,
                "w": 2,
                "h": 5,
                "i": "b437c117-9f35-44ce-b544-bd1e83150f16",
                "c": "SocketWidget",
                "settings": {
                "title": {
                    "val": "CPU Temp",
                    "type": "input"
                },
                "showTitle": {
                    "val": true,
                    "type": "checkbox"
                }
                },
                "moved": false
            }
        ]"#;

    // Parse the string of data into serde_json::Value.
    let settings: Value = serde_json::from_str(settings).expect("Failed to parse JSON input");

    connection_pool.create_dashboard_for_user(&user, &"blabla".to_string(), &settings);
}
