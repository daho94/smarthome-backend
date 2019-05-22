use crate::api::auth::handler::LoggedUser;
use crate::errors::ServiceError;
use crate::models::{Dashboard, DbExecutor};
use actix::{Handler, Message};
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct GetDashboards {
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct DashboardMeta {
    pub id: i32,
    pub name: String,
    pub is_default: bool,
}

impl Message for GetDashboards {
    type Result = Result<Vec<DashboardMeta>, ServiceError>;
}

impl From<LoggedUser> for GetDashboards {
    fn from(logged_user: LoggedUser) -> Self {
        GetDashboards {
            username: logged_user.username,
        }
    }
}

impl Handler<GetDashboards> for DbExecutor {
    type Result = Result<Vec<DashboardMeta>, ServiceError>;
    fn handle(&mut self, msg: GetDashboards, _: &mut Self::Context) -> Self::Result {
        self.0
            .get_user(&msg.username)
            .and_then(|user| self.0.get_dashboards_for_user(&user))
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
            .map_err(|_e| ServiceError::InternalServerError)
    }
}

#[derive(Debug, Deserialize)]
pub struct DashboardData {
    pub id: i32,
}

pub struct GetDashboard {
    pub username: String,
    pub dashboard_id: i32,
}

impl Message for GetDashboard {
    type Result = Result<Dashboard, ServiceError>;
}

impl Handler<GetDashboard> for DbExecutor {
    type Result = Result<Dashboard, ServiceError>;
    fn handle(&mut self, msg: GetDashboard, _: &mut Self::Context) -> Self::Result {
        self.0
            .get_user(&msg.username)
            .and_then(|user| self.0.get_dashboard_for_user(msg.dashboard_id, &user))
            .map_err(|_e| ServiceError::InternalServerError)
    }
}

pub struct GetDefaultDashboard {
    pub username: String,
}

impl Message for GetDefaultDashboard {
    type Result = Result<DashboardMeta, ServiceError>;
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
 

impl Handler<GetDefaultDashboard> for DbExecutor {
    type Result = Result<DashboardMeta, ServiceError>;
    fn handle(&mut self, msg: GetDefaultDashboard, _: &mut Self::Context) -> Self::Result {
        self.0
            .get_user(&msg.username)
            .and_then(|user| self.0.get_default_dashboard_for_user(&user))
            .and_then(|dashboard| Ok(dashboard.into()))
            .map_err(|_e| ServiceError::InternalServerError)
    }
}

#[derive(Debug)]
pub struct SaveDashboard {
    pub username: String,
    pub id: i32,
    pub new_settings: Value,
}

#[derive(Debug, Deserialize)]
pub struct DashboardSettings {
    pub id: i32,
    pub settings: Value,
}

impl Message for SaveDashboard {
    type Result = Result<DashboardMeta, ServiceError>;
}

impl Handler<SaveDashboard> for DbExecutor {
    type Result = Result<DashboardMeta, ServiceError>;
    fn handle(&mut self, msg: SaveDashboard, _: &mut Self::Context) -> Self::Result {
        self.0
            .get_user(&msg.username)
            .and_then(|user| Ok(self.0.save_dashboard_for_user(&user, msg.id, &msg.new_settings).into()))
            .map_err(|_e| ServiceError::InternalServerError)
    }
}