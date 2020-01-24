use crate::models::{DashboardFolder, Tree};
use actix_web::web;
use database::ConnectionPool;
use diesel::result::Error;

pub fn get_tree(pool: web::Data<ConnectionPool>, username: &str) -> Result<Tree, Error> {
    pool.get_user(username)
        .and_then(|user| pool.get_folder_tree(&user))
}

#[derive(Debug, Deserialize)]
pub struct CreateFolder {
    pub name: String,
    pub icon: String,
    pub parent_id: i32,
}

pub fn create_folder(
    pool: web::Data<ConnectionPool>,
    folder: &CreateFolder,
    username: &str,
) -> Result<DashboardFolder, Error> {
    pool.get_user(username).and_then(|user| {
        Ok(pool.create_folder(&folder.name, folder.parent_id, &folder.icon, &user))
    })
}
