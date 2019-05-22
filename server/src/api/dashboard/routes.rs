use super::handler::{DashboardData, GetDashboard, GetDashboards, GetDefaultDashboard, SaveDashboard, DashboardSettings};
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

pub fn get_dashboard(
    logged_user: LoggedUser,
    dashboard_data: web::Json<DashboardData>,
    db: web::Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(GetDashboard {
        username: logged_user.username,
        dashboard_id: dashboard_data.id,
    })
    .from_err()
    .and_then(move |res| match res {
        Ok(dashboard) => Ok(HttpResponse::Ok().json(dashboard)),
        Err(e) => Ok(e.error_response()),
    })
}

pub fn get_default_dashboard(
    logged_user: LoggedUser,
    db: web::Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(GetDefaultDashboard {
        username: logged_user.username,
    })
    .from_err()
    .and_then(move |res| match res {
        Ok(dashboard) => Ok(HttpResponse::Ok().json(dashboard)),
        Err(e) => Ok(e.error_response()),
    })
}

pub fn save_dashboard(
    logged_user: LoggedUser,
    dashboard_data: web::Json<DashboardSettings>,
    db: web::Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
     db.send(SaveDashboard {
         username: logged_user.username,
         id: dashboard_data.id,
         new_settings: dashboard_data.settings.clone(),
     })
    .from_err()
    .and_then(move |res| match res {
        Ok(_) => Ok(HttpResponse::Ok().into()),
        Err(e) => Ok(e.error_response()),
    })
}
