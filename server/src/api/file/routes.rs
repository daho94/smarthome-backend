use super::handler::{download_file_from_uri, save_file, FileDownload};
use crate::api::auth::handler::LoggedUser;
use actix_multipart::Multipart;
use actix_web::{error, web, Error, HttpResponse, Responder};
use futures::{Future, Stream};

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
    download_file_from_uri(&args);
    //Ok(_) => HttpResponse::Ok().into(),
    //Err(_) => InternalServerError.error_response(),
    HttpResponse::Ok()
}
