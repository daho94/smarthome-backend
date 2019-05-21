use crate::api::auth::routes::{get_me, login, logout};
use crate::api::dashboard::routes::*;
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
            .service(
                web::scope("/dashboard")
                    .service(web::resource("/all").route(web::get().to_async(get_dashboards)))
                    .service(
                        web::resource("/single")
                            .route(web::post().to_async(get_dashboard))
                            .route(web::get().to_async(get_default_dashboard)),
                    ),
            ),
    )
    .service(fs::Files::new("/", "./web/").index_file("index.html"));
}
