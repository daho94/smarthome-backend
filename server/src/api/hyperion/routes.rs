use super::handler::{Command, HyperionCmd, HyperionCmdAddr};
use crate::api::auth::handler::LoggedUser;
use crate::errors::ServiceError::InternalServerError;
use actix_web::{web, HttpResponse, Responder, ResponseError};
use hyperion_rs::palette::{Component, Srgb};
use hyperion_rs::Hyperion;
use serde_json::json;

pub fn get_effects(_logged_user: LoggedUser, args: web::Json<HyperionCmdAddr>) -> impl Responder {
    let cmd = args.into_inner();
    let hyperion = Hyperion::new(&cmd.address);

    if let Some(effects) = hyperion.get_effects() {
        HttpResponse::Ok().json(json!({ "effects": effects }))
    } else {
        InternalServerError.error_response()
    }
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

pub fn clear_all(_logged_user: LoggedUser, args: web::Json<HyperionCmdAddr>) -> impl Responder {
    let cmd = args.into_inner();
    let hyperion = Hyperion::new(&cmd.address);

    match hyperion.clear_all() {
        Ok(_) => HttpResponse::Ok().into(),
        Err(_) => InternalServerError.error_response(),
    }
}
