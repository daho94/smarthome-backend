use crate::api::auth::routes::*;
use crate::api::dashboard::routes::*;
use crate::api::iobroker::routes::*;
use crate::api::widget::routes::*;
use actix_files as fs;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    // configure default routes
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/auth")
                    .route(web::post().to_async(login))
                    .route(web::delete().to(logout))
                    .route(web::get().to_async(get_me)),
            )
            .service(web::resource("/iobroker").route(web::post().to_async(get_datapoints)))
            .service(
                web::scope("/iobroker")
                    .service(
                        web::resource("/datapoints").route(web::post().to_async(get_datapoints)),
                    )
                    .service(
                        web::resource("/history")
                            .route(web::post().to_async(get_datapoint_history_availability)),
                    ),
            )
            .service(
                web::scope("/dashboard")
                    .service(web::resource("/all").route(web::get().to_async(get_dashboards)))
                    .service(
                        web::resource("/single")
                            .route(web::post().to_async(get_dashboard))
                            .route(web::get().to_async(get_default_dashboard))
                            .route(web::put().to_async(save_dashboard)),
                    )
                    .service(
                        web::resource("/create").route(web::post().to_async(create_dashboard)),
                    ),
            )
            .service(web::resource("/widget/all").route(web::get().to_async(get_widgets))),
    )
    .service(fs::Files::new("/", "./web/").index_file("index.html"));

    // configure feature based routes
    #[cfg(feature = "funksteckdose")]
    enable_socket_control(&mut cfg);
}

#[cfg(feature = "funksteckdose")]
fn enable_socket_control(cfg: &mut web::ServiceConfig) {
    use crate::api::socket::routes::*;

    cfg.service(web::resource("/api/socket").route(web::post().to(control_socket)));
}
