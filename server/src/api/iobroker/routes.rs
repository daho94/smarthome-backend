use super::handler::{get_datapoints_in_interval, IobrokerState};
use crate::api::auth::handler::LoggedUser;
use actix_web::{web, Error, HttpResponse, ResponseError};
use database::ConnectionPool;
use futures::Future;

pub fn get_datapoints(
    _logged_user: LoggedUser,
    state: web::Json<IobrokerState>,
    pool: web::Data<ConnectionPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || get_datapoints_in_interval(pool, &state.name, state.from, state.to)).then(
        |res| match res {
            Ok(datapoints) => Ok(HttpResponse::Ok().json(datapoints)),
            Err(e) => Ok(e.error_response()),
        },
    )
}
