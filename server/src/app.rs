use crate::api::auth::routes::{get_me, login, logout};
use actix_files as fs;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::resource("/auth")
                .route(web::post().to_async(login))
                .route(web::delete().to(logout))
                .route(web::get().to_async(get_me)),
        ),
    )
    //     .service(
    //         web::scope("/releasenotes/{name}")
    //             .service(
    //                 web::resource("/versions")
    //                     .route(web::get().to(releasenotes::get_versions)),
    //             )
    //             .service(
    //                 web::resource("/issues")
    //                     .route(web::get().to_async(releasenotes::get_issues)),
    //             ),
    //     )
    //     .service(web::resource("/user").route(web::post().to_async(users::create_user)))
    //     .service(web::resource("/group").route(web::post().to_async(groups::create_group)))
    //     .service(
    //         web::resource("/usergroup")
    //             .route(web::post().to_async(usergroups::create_user_group_relation))
    //             .route(web::get().to_async(usergroups::get_groups_for_user)),
    //     ),
    //     ),
    // )
    // .service(web::resource("/").route(web::get().to(|| {
    //     HttpResponse::Found()
    //         .header("LOCATION", "/index.html")
    //         .finish()
    // })))
    .service(fs::Files::new("/", "./web/").index_file("index.html"));
}
