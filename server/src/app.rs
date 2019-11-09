use crate::api::auth::routes as auth;
use crate::api::dashboard::routes as dashboard;
use crate::api::hyperion::routes as hyperion;
use crate::api::iobroker::routes as iobroker;
use crate::api::socket::routes as socket;
use crate::api::widget::routes as widget;
use crate::api::file::routes as file;
use actix_files as fs;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::resource("/auth")
                    .route(web::post().to_async(auth::login))
                    .route(web::delete().to(auth::logout))
                    .route(web::get().to_async(auth::get_me)),
            )
            .service(
                web::scope("/iobroker")
                    .service(
                        web::resource("/datapoints")
                            .route(web::post().to_async(iobroker::get_datapoints)),
                    )
                    .service(
                        web::resource("/history").route(
                            web::post().to_async(iobroker::get_datapoint_history_availability),
                        ),
                    ),
            )
            .service(
                web::scope("/hyperion")
                    .service(
                        web::resource("/command").route(web::post().to(hyperion::send_command)),
                    )
                    .service(web::resource("/effects").route(web::post().to(hyperion::get_effects)))
                    .service(web::resource("/clear").route(web::post().to(hyperion::clear_all))),
            )
            .service(
                web::scope("/dashboard")
                    .service(
                        web::resource("/all").route(web::get().to_async(dashboard::get_dashboards)),
                    )
                    .service(
                        web::resource("/single")
                            .route(web::post().to_async(dashboard::get_dashboard))
                            .route(web::get().to_async(dashboard::get_default_dashboard))
                            .route(web::put().to_async(dashboard::save_dashboard))
                            .route(web::delete().to_async(dashboard::delete_dashboard)),
                    )
                    .service(
                        web::resource("/create")
                            .route(web::post().to_async(dashboard::create_dashboard)),
                    ),
            )
            .service(web::resource("/widget/all").route(web::get().to_async(widget::get_widgets)))
            .service(web::resource("/socket").route(web::post().to(socket::control_socket)))
            .service(web::resource("/file").route(web::post().to_async(file::upload)))
    )
    .service(fs::Files::new("/", "./web/").index_file("index.html"));
}
