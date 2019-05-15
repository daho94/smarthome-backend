use actix::{Actor, SyncContext};
use database::PgConnectionPool;
use database::models::user::User;

pub struct DbExecutor(pub PgConnectionPool);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub username: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser { username: user.username }
    }
}