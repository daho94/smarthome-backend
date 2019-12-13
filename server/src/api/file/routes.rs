use super::handler::{download_file_from_uri, get_files, save_file, FileDownload};
use crate::api::auth::handler::LoggedUser;
use crate::errors::ServiceError::InternalServerError;
use actix_multipart::Multipart;
use actix_web::{error, web, Error, HttpResponse, Responder, ResponseError};
use futures::{Future, Stream};
use serde_json::json;

pub fn upload(
    multipart: Multipart,
    _logged_user: LoggedUser,
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

pub fn upload_from_uri(args: web::Json<FileDownload>, _logged_user: LoggedUser) -> impl Responder {
    let args = args.into_inner();
    match download_file_from_uri(&args) {
        Ok(sizes) => HttpResponse::Ok().json(sizes),
        Err(e) => HttpResponse::Ok().json(json!({ "message": e })),
    }
}
pub fn get_uploaded_files(_logged_user: LoggedUser) -> impl Responder {
    match get_files() {
        Ok(files) => HttpResponse::Ok().json(files),
        Err(_) => InternalServerError.error_response(),
    }
}
