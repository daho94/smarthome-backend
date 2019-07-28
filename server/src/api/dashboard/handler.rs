use crate::models::Dashboard;
use actix_web::web;
use database::ConnectionPool;
use diesel::result::Error;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct CreateDashboard {
    pub name: String,
    pub icon: String,
}

pub fn create_dashboard_for_user(
    pool: web::Data<ConnectionPool>,
    dashboard: &CreateDashboard,
    username: &str,
) -> Result<DashboardMeta, Error> {
    pool.get_user(username).and_then(|user| {
        let settings = r#"[]"#;
        let settings: Value = serde_json::from_str(settings).expect("Failed to parse JSON input");
        Ok(pool
            .create_dashboard_for_user(&user, &dashboard.name, &settings, false, &dashboard.icon)
            .into())
    })
}

#[derive(Debug, Serialize)]
pub struct DashboardMeta {
    pub id: i32,
    pub name: String,
    pub is_default: bool,
    pub icon: String,
}

impl From<Dashboard> for DashboardMeta {
    fn from(dashboard: Dashboard) -> Self {
        DashboardMeta {
            id: dashboard.id,
            name: dashboard.name,
            is_default: dashboard.default_dashboard,
            icon: dashboard.icon,
        }
    }
}

pub fn get_dashboards_for_user(
    pool: web::Data<ConnectionPool>,
    username: &str,
) -> Result<Vec<DashboardMeta>, Error> {
    pool.get_user(username)
        .and_then(|user| pool.get_dashboards_for_user(&user))
        .and_then(|dashboards| {
            Ok(dashboards
                .iter()
                .map(|x| DashboardMeta {
                    id: x.0,
                    name: x.1.to_owned(),
                    is_default: x.2,
                    icon: x.3.to_owned(),
                })
                .collect())
        })
}

#[derive(Debug, Deserialize)]
pub struct DashboardData {
    pub id: i32,
}

pub fn get_dashboard_by_id(
    pool: web::Data<ConnectionPool>,
    dashboard_id: i32,
    username: &str,
) -> Result<Dashboard, Error> {
    pool.get_user(username)
        .and_then(|user| pool.get_dashboard_for_user(dashboard_id, &user))
}

pub fn delete_dashboard_by_id(
    pool: web::Data<ConnectionPool>,
    dashboard_id: i32,
) -> Result<usize, Error> {
    Ok(pool.delete_dashboard_by_id(dashboard_id))
}

pub fn get_default_dashboard_for_user(
    pool: web::Data<ConnectionPool>,
    username: &str,
) -> Result<DashboardMeta, Error> {
    pool.get_user(username)
        .and_then(|user| pool.get_default_dashboard_for_user(&user))
        .and_then(|dashboard| Ok(dashboard.into()))
}

#[derive(Debug, Deserialize)]
pub struct DashboardSettings {
    pub id: i32,
    pub settings: Value,
}

pub fn save_dashboard_for_user(
    pool: web::Data<ConnectionPool>,
    msg: &DashboardSettings,
    username: &str,
) -> Result<DashboardMeta, Error> {
    pool.get_user(username).and_then(|user| {
        Ok(pool
            .save_dashboard_for_user(&user, msg.id, &msg.settings)
            .into())
    })
}
