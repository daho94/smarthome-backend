use super::handler::{
    get_datapoints_in_interval, has_datapoint_history, HistoryAvailability, IobrokerState,
    IobrokerStateHistory,
};
use crate::api::auth::handler::LoggedUser;
use actix_web::{web, Error, HttpResponse, ResponseError};
use database::ConnectionPool;
use futures::Future;

pub fn get_datapoints(
    _logged_user: LoggedUser,
    state: web::Json<IobrokerStateHistory>,
    pool: web::Data<ConnectionPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || get_datapoints_in_interval(pool, &state.name, state.from, state.to)).then(
        |res| match res {
            Ok(datapoints) => Ok(HttpResponse::Ok().json(datapoints)),
            Err(e) => Ok(e.error_response()),
        },
    )
}

pub fn get_datapoint_history_availability(
    _logged_user: LoggedUser,
    state: web::Json<IobrokerState>,
    pool: web::Data<ConnectionPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || has_datapoint_history(pool, &state.name)).then(|res| match res {
        Ok(av) => Ok(HttpResponse::Ok().json(av)),
        Err(_) => Ok(HttpResponse::Ok().json(HistoryAvailability { has_history: false })),
    })
}
