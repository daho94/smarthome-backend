use super::handler::get_widgets_grouped;
use crate::api::auth::handler::LoggedUser;
use database::ConnectionPool;
use actix_web::{web, Error, HttpResponse, ResponseError};
use futures::Future;

pub fn get_widgets(
    _logged_user: LoggedUser,
    pool: web::Data<ConnectionPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || get_widgets_grouped(pool)).then(|res| match res {
        Ok(widgets) => Ok(HttpResponse::Ok().json(widgets)),
        Err(e) => Ok(e.error_response()),
    })
}