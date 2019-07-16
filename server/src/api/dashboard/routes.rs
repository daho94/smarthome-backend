use super::handler::{
    create_dashboard_for_user, get_dashboard_by_id, get_dashboards_for_user,
    get_default_dashboard_for_user, save_dashboard_for_user,
};
use super::handler::{CreateDashboard, DashboardData, DashboardSettings};
use crate::api::auth::handler::LoggedUser;
use actix_web::{web, Error, HttpResponse, ResponseError};
use database::ConnectionPool;
use futures::Future;

pub fn get_dashboards(
    logged_user: LoggedUser,
    pool: web::Data<ConnectionPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || get_dashboards_for_user(pool, &logged_user.username)).then(|res| match res {
        Ok(dashboards) => Ok(HttpResponse::Ok().json(dashboards)),
        Err(e) => Ok(e.error_response()),
    })
}

pub fn get_dashboard(
    logged_user: LoggedUser,
    dashboard_data: web::Json<DashboardData>,
    pool: web::Data<ConnectionPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || get_dashboard_by_id(pool, dashboard_data.id, &logged_user.username)).then(
        |res| match res {
            Ok(dashboard) => Ok(HttpResponse::Ok().json(dashboard)),
            Err(e) => Ok(e.error_response()),
        },
    )
}

pub fn get_default_dashboard(
    logged_user: LoggedUser,
    pool: web::Data<ConnectionPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || get_default_dashboard_for_user(pool, &logged_user.username)).then(|res| {
        match res {
            Ok(dashboard) => Ok(HttpResponse::Ok().json(dashboard)),
            Err(e) => Ok(e.error_response()),
        }
    })
}

pub fn save_dashboard(
    logged_user: LoggedUser,
    dashboard_data: web::Json<DashboardSettings>,
    pool: web::Data<ConnectionPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        save_dashboard_for_user(pool, &dashboard_data.into_inner(), &logged_user.username)
    })
    .then(|res| match res {
        Ok(_) => Ok(HttpResponse::Ok().into()),
        Err(e) => Ok(e.error_response()),
    })
}
pub fn create_dashboard(
    logged_user: LoggedUser,
    dashboard_data: web::Json<CreateDashboard>,
    pool: web::Data<ConnectionPool>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        create_dashboard_for_user(pool, &dashboard_data.into_inner(), &logged_user.username)
    })
    .then(|res| match res {
        Ok(_) => Ok(HttpResponse::Ok().into()),
        Err(e) => Ok(e.error_response()),
    })
}
