use actix::Addr;
use actix_web::middleware::identity::Identity;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder, ResponseError};
use futures::Future;

use super::handler::{AuthData, LoggedUser};
use super::utils::create_token;
use crate::errors::ServiceError;
use crate::models::DbExecutor;

pub fn login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    db: web::Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(auth_data.into_inner())
        .from_err()
        .and_then(move |res| match res {
            Ok(user) => {
                let jwt = create_token(&user)?;
                id.remember(jwt);
                Ok(HttpResponse::Ok().into())
            }
            Err(e) => Ok(e.error_response()),
        })
}

pub fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok()
}

pub fn get_me(logged_user: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}
