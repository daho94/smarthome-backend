use crate::models::user::User;
use diesel::prelude::*;

pub type Result<T> = std::result::Result<T, diesel::result::Error>;

pub fn get_user(conn: &PgConnection, name: &str) -> Result<User> {
    use crate::schema::users::dsl::*;

    users.filter(username.eq(name)).first::<User>(conn)
}
