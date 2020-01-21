use super::handler::get_tree;
use crate::api::auth::handler::LoggedUser;
use actix_web::{web, Error, HttpResponse, ResponseError};
use database::ConnectionPool;
use futures::Future;

pub fn get_folder_tree(
    logged_user: LoggedUser,
    pool: web::Data<ConnectionPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || get_tree(pool, &logged_user.username)).then(|res| match res {
        Ok(dashboards) => Ok(HttpResponse::Ok().json(dashboards)),
        Err(e) => Ok(e.error_response()),
    })
}
