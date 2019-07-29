use crate::api::auth::handler::LoggedUser;
use actix_web::{HttpResponse, Responder};

#[cfg(feature = "wiring-pi")]
pub fn control_socket(
    _logged_user: LoggedUser,
    args: actix_web::web::Json<super::handler::ControlArgs>,
) -> impl Responder {
    super::handler::send_socket_state(&args.into_inner());
    HttpResponse::Ok()
}

// not supported on non-raspberrypi devices
#[cfg(not(feature = "wiring-pi"))]
pub fn control_socket(_logged_user: LoggedUser) -> impl Responder {
    use serde_json::json;

    HttpResponse::ServiceUnavailable().json(
        json!({"message": "This function is only supported on arm-devices running wiring-pi"}),
    )
}
