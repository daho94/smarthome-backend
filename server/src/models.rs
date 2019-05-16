use actix::{Actor, SyncContext};
use database::models::user::User;
use database::{ConnectionPool, PgConnectionPool};

pub struct DbExecutor(pub ConnectionPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

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
