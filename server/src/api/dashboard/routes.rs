use super::handler::GetDashboards;
use crate::api::auth::handler::LoggedUser;
use crate::models::DbExecutor;
use actix::Addr;
use actix_web::{web, Error, HttpResponse, ResponseError};
use futures::Future;

pub fn get_dashboards(
    logged_user: LoggedUser,
    db: web::Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(GetDashboards::from(logged_user))
        .from_err()
        .and_then(move |res| match res {
            Ok(dashboards) => Ok(HttpResponse::Ok().json(dashboards)),
            Err(e) => Ok(e.error_response()),
        })
}
