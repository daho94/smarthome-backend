use crate::models::user::*;
use crate::ConnectionPool;
use diesel::prelude::*;

pub type Result<T> = std::result::Result<T, diesel::result::Error>;

impl ConnectionPool {
    pub fn get_user(&self, name: &str) -> Result<User> {
        use crate::schema::users::dsl::*;
        let conn = self.connection();
        users.filter(username.eq(name)).first::<User>(&conn)
    }

    pub fn create_user(&self, username: &str, password: &str) -> User {
        use crate::schema::users;

        let conn = self.connection();
        let new_user = NewUser { username, password };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&conn)
            .expect("Error saving new user")
    }
}
