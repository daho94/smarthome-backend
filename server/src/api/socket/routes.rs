use crate::api::auth::handler::LoggedUser;
use actix_web::{web, HttpResponse, Responder};

#[cfg(feature = "wiring-pi")]
pub fn control_socket(_logged_user: LoggedUser, args: web::Json<ControlArgs>) -> impl Responder {
    use super::handler::{send_socket_state, ControlArgs};


    send_socket(&args.into_inner());
    HttpResponse::Ok()
}

// not supported on device non-raspberrypi devices
pub fn control_socket(_logged_user: LoggedUser) -> impl Responder {
    HttpResponse:: ServiceUnavailable()
}