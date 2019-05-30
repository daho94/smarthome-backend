use super::handler::{login_user, AuthData, LoggedUser};
use super::utils::create_token;
use actix_web::middleware::identity::Identity;
use actix_web::{web, Error, HttpResponse, Responder, ResponseError};
use database::ConnectionPool;
use futures::Future;

pub fn login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    pool: web::Data<ConnectionPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || login_user(pool, &auth_data.into_inner())).then(move |res| match res {
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
