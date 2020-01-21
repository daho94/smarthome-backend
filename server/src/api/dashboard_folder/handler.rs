use crate::models::Tree;
use actix_web::web;
use database::ConnectionPool;
use diesel::result::Error;

pub fn get_tree(pool: web::Data<ConnectionPool>, username: &str) -> Result<Tree, Error> {
    pool.get_user(username)
        .and_then(|user| pool.get_folder_tree(&user))
}
