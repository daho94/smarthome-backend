pub use database::models::dashboard::Dashboard;
use database::models::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub username: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            username: user.username,
        }
    }
}

impl From<&User> for SlimUser {
    fn from(user: &User) -> Self {
        SlimUser {
            username: user.username.clone(),
        }
    }
}
