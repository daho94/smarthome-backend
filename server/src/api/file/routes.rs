use crate::api::auth::handler::LoggedUser;
use super::handler::save_file;
use actix_multipart::{Multipart};
use actix_web::{error, Error, HttpResponse};
use futures::{Future, Stream};

pub fn upload(
    multipart: Multipart,
    _logged_user: LoggedUser
) -> impl Future<Item = HttpResponse, Error = Error> {
    multipart
        .map_err(error::ErrorInternalServerError)
        .map(|field| save_file(field).into_stream())
        .flatten()
        .collect()
        .map(|sizes| HttpResponse::Ok().json(sizes))
        .map_err(|e| {
            println!("failed: {}", e);
            e
        })
}