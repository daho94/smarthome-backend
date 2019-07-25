use crate::api::auth::routes::*;
use crate::api::dashboard::routes::*;
use crate::api::hyperion::routes as hyperion;
use crate::api::iobroker::routes::*;
use crate::api::socket::routes::*;
use crate::api::widget::routes::*;
use actix_files as fs;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
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
                web::scope("/hyperion").service(
                    web::resource("/command").route(web::post().to(hyperion::send_command)),
                ),
            )
            .service(
                web::scope("/dashboard")
                    .service(web::resource("/all").route(web::get().to_async(get_dashboards)))
                    .service(
                        web::resource("/single")
                            .route(web::post().to_async(get_dashboard))
                            .route(web::get().to_async(get_default_dashboard))
                            .route(web::put().to_async(save_dashboard))
                            .route(web::delete().to_async(delete_dashboard)),
                    )
                    .service(
                        web::resource("/create").route(web::post().to_async(create_dashboard)),
                    ),
            )
            .service(web::resource("/widget/all").route(web::get().to_async(get_widgets)))
            .service(web::resource("/socket").route(web::post().to(control_socket))),
    )
    .service(fs::Files::new("/", "./web/").index_file("index.html"));
}
