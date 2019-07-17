use super::handler::{send_socket_state, ControlArgs};
use crate::api::auth::handler::LoggedUser;
use actix_web::{web, HttpResponse, Responder};

pub fn control_socket(_logged_user: LoggedUser, args: web::Json<ControlArgs>) -> impl Responder {
    send_socket(&args.into_inner());
    HttpResponse::Ok()
}
