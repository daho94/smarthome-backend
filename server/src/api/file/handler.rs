use actix_multipart::{Field, MultipartError};
use actix_web::{error, web, Error};
use futures::future::{err, Either};
use futures::{Future, Stream};
use std::fs;
use std::io::{self, copy, Write};
use std::path::Path;

const UPLOAD_DIR: &str = "web/upload/";

fn try_create_upload_dir() -> io::Result<()> {
    if !Path::new(UPLOAD_DIR).is_dir() {
        fs::create_dir(UPLOAD_DIR)?
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct FileDownload {
    uri: String,
    file_name: String,
}

fn is_file_already_existent(path: &Path) -> bool {
    Path::new(path).exists()
}

pub fn download_file_from_uri(file: &FileDownload) -> Result<u64, &'static str> {
    let mut response = reqwest::blocking::get(&file.uri).unwrap();

    if try_create_upload_dir().is_err() {
        return Err("Failed to create upload directory");
    }

    if is_file_already_existent(Path::new(&format!("{}{}", UPLOAD_DIR, file.file_name))) {
        return Ok(0);
    }

    fs::File::create(format!("{}{}", UPLOAD_DIR, file.file_name))
        .and_then(|mut out| copy(&mut response, &mut out))
        .map_err(|_| "Failed to write file")
}

pub fn save_file(field: Field) -> impl Future<Item = i64, Error = Error> {
    let file_path_string = format!(
        "{}{}",
        UPLOAD_DIR,
        field
            .content_disposition()
            .expect("Failed to parse file")
            .get_filename()
            .expect("Failed to parse file")
    );

    if let Err(e) = try_create_upload_dir() {
        return Either::A(err(error::ErrorInternalServerError(e)));
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
