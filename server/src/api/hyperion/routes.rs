use super::handler::{Command, HyperionCmd};
use crate::api::auth::handler::LoggedUser;
use crate::errors::ServiceError::InternalServerError;
use actix_web::{web, HttpResponse, Responder, ResponseError};
use hyperion_rs::palette::{Component, Srgb};
use hyperion_rs::Hyperion;

pub fn set_effect(_logged_user: LoggedUser) -> impl Responder {
    unimplemented!()
}

pub fn get_effects(_logged_user: LoggedUser) -> impl Responder {
    unimplemented!()
}

pub fn set_color(_logged_user: LoggedUser) -> impl Responder {
    unimplemented!()
}

pub fn send_command(_logged_user: LoggedUser, args: web::Json<HyperionCmd>) -> impl Responder {
    let cmd = args.into_inner();
    let hyperion = Hyperion::new(&cmd.address);

    let res = match cmd.command {
        Command::Effect { name } => hyperion.set_effect(&name),
        Command::Color { r, g, b } => {
            let srgb = Srgb::new(r.convert(), g.convert(), b.convert()).into_linear();
            hyperion.set_color(srgb.into())
        }
    };

    match res {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => InternalServerError.error_response(),
    }
}

pub fn clear_all(_logged_user: LoggedUser) -> impl Responder {
    unimplemented!()
}
