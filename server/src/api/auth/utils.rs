use crate::errors::ServiceError;
use crate::models::{DbExecutor, SlimUser};
use chrono::{Duration, Local};
use frank_jwt::{decode, encode, validate_signature, Algorithm};
use serde_json::json;

const ALGO: Algorithm = Algorithm::HS512;

pub fn create_token(user: &SlimUser) -> Result<String, ServiceError> {
    let claims = Claims::with_username(&user.username);
    let payload = serde_json::to_value(claims).unwrap();
    encode(json!({}), &get_secret(), &payload, ALGO).map_err(|e| {
        eprint!("{}", e);
        ServiceError::InternalServerError
    })
}

pub fn validate_token(jwt: &str) -> Result<bool, ServiceError> {
    validate_signature(&jwt.to_string(), &get_secret(), ALGO).map_err(|e| {
        eprint!("{}", e);
        ServiceError::Unauthorized
    })
}

pub fn decode_token(jwt: &str) -> Result<SlimUser, ServiceError> {
    let (_header, payload) = decode(&jwt.to_string(), &get_secret(), ALGO).map_err(|e| {
        eprint!("{}", e);
        ServiceError::InternalServerError
    })?;
    let user: SlimUser = serde_json::from_value(payload).map_err(|e| {
        eprint!("{}", e);
        ServiceError::InternalServerError
    })?;
    Ok(user)
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // issuer
    iss: String,
    // subject
    sub: String,
    //issued at
    iat: i64,
    // expiry
    exp: i64,
    // user email
    username: String,
}

// struct to get converted to token and back
impl Claims {
    fn with_username(username: &str) -> Self {
        Claims {
            iss: "localhost".into(),
            sub: "auth".into(),
            username: username.to_owned(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
        }
    }
}

impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser {
            username: claims.username,
        }
    }
}

fn get_secret() -> String {
    std::env::var("JWT_SECRET").unwrap_or_else(|_| "my secret".into())
}
