use crate::api::auth::routes::{get_me, login, logout};
use crate::api::dashboard::routes::get_dashboards;
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
            .service(web::resource("/dashboard").route(web::get().to_async(get_dashboards))),
    )
    .service(fs::Files::new("/", "./web/").index_file("index.html"));
}
