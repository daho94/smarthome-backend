use crate::models::Dashboard;
use serde_json::Value;
use database::ConnectionPool;
use actix_web::web;
use diesel::result::Error;

#[derive(Debug, Serialize)]
pub struct DashboardMeta {
    pub id: i32,
    pub name: String,
    pub is_default: bool,
}

impl From<Dashboard> for DashboardMeta {
    fn from(dashboard: Dashboard) -> Self {
        DashboardMeta {
            id: dashboard.id,
            name: dashboard.name,
            is_default: dashboard.default_dashboard,
        }
    }
}

pub fn get_dashboards_for_user(pool: web::Data<ConnectionPool>, username: &str) -> Result<Vec<DashboardMeta>, Error> {
    pool            
        .get_user(username)
        .and_then(|user| pool.get_dashboards_for_user(&user))
        .and_then(|dashboards| {
            Ok(dashboards
                .iter()
                .map(|x| DashboardMeta {
                    id: x.0,
                    name: x.1.to_owned(),
                    is_default: x.2,
                })
                .collect())
        })
}

#[derive(Debug, Deserialize)]
pub struct DashboardData {
    pub id: i32,
}

pub fn get_dashboard_by_id(pool: web::Data<ConnectionPool>, dashboard_id: i32, username: &str) -> Result<Dashboard, Error> {
    pool
        .get_user(username)
        .and_then(|user| pool.get_dashboard_for_user(dashboard_id, &user))
}

pub fn get_default_dashboard_for_user(pool: web::Data<ConnectionPool>, username: &str) -> Result<DashboardMeta, Error> {
    pool
        .get_user(username)
        .and_then(|user| pool.get_default_dashboard_for_user(&user))
        .and_then(|dashboard| Ok(dashboard.into()))
}

#[derive(Debug, Deserialize)]
pub struct DashboardSettings {
    pub id: i32,
    pub settings: Value,
}

pub fn save_dashboard_for_user(pool: web::Data<ConnectionPool>, msg: &DashboardSettings, username: &str) -> Result<DashboardMeta, Error> {
    pool
        .get_user(username)
        .and_then(|user| Ok(pool.save_dashboard_for_user(&user, msg.id, &msg.settings).into()))
}