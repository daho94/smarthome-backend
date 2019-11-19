use actix_multipart::{Field, MultipartError};
use actix_web::{error, web, Error};
use futures::future::{err, Either};
use futures::{Future, Stream};
use std::fs;
use std::io::{copy, Write};
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct FileDownload {
    uri: String,
    file_name: String,
}

//FIXME: check if file already exists? check if dir exists
pub fn download_file_from_uri(file: &FileDownload) {
    let mut response = reqwest::blocking::get(&file.uri).expect("Request failed");
    let mut out =
        fs::File::create(format!("web/upload/{}", file.file_name)).expect("faield to create file");
    copy(&mut response, &mut out).expect("failed to copy content");
}

//FIXME: Decide where to upload files
pub fn save_file(field: Field) -> impl Future<Item = i64, Error = Error> {
    let file_path_string = format!(
        "web/upload/{}",
        field
            .content_disposition()
            .expect("Failed to parse file")
            .get_filename()
            .expect("Failed to parse file")
    );

    if Path::new("web/upload/").is_dir() == false {
        match fs::create_dir("web/upload") {
            Ok(()) => { /* it worked */ }
            Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
        }
    }

    let file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };

    Either::B(
        field
            .fold((file, 0i64), move |(mut file, mut acc), bytes| {
                // fs operations are blocking, we have to execute writes
                // on threadpool
                web::block(move || {
                    file.write_all(bytes.as_ref()).map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        MultipartError::Payload(error::PayloadError::Io(e))
                    })?;
                    acc += bytes.len() as i64;
                    Ok((file, acc))
                })
                .map_err(|e: error::BlockingError<MultipartError>| match e {
                    error::BlockingError::Error(e) => e,
                    error::BlockingError::Canceled => MultipartError::Incomplete,
                })
            })
            .map(|(_, acc)| acc)
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}
