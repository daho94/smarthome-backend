pub use database::actions::dashboard_folder::Tree;
pub use database::models::dashboard::Dashboard;
pub use database::models::dashboard_folder::DashboardFolder;
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
