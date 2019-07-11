use crate::api::auth::utils::decode_token;
use crate::errors::ServiceError;
use crate::models::SlimUser;
use actix_web::web;
use actix_web::{dev::Payload, Error, HttpRequest};
use actix_web::FromRequest;
use actix_identity::Identity;
use database::ConnectionPool;
use bcrypt::{verify};

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub username: String,
    pub password: String,
}

pub fn login_user(
    pool: web::Data<ConnectionPool>,
    auth_data: &AuthData,
) -> Result<SlimUser, ServiceError> {
    if let Ok(user) = pool.get_user(&auth_data.username) {
        if let Ok(matching) = verify(&auth_data.password, &user.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::BadRequest(
        "Username and Password don't match".into(),
    ))
}

// we need the same data
// simple aliasing makes the intentions clear and its more readable
pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Config = ();
    type Error = Error;
    type Future = Result<LoggedUser, Error>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Some(identity) = Identity::from_request(req, pl)?.identity() {
            let user: SlimUser = decode_token(&identity)?;
            return Ok(user as LoggedUser);
        }
        Err(ServiceError::Unauthorized.into())
    }
}
