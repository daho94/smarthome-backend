use crate::api::auth::handler::LoggedUser;
use crate::errors::ServiceError;
use crate::models::{Dashboard, DbExecutor};
use actix::{Handler, Message};

#[derive(Debug, Deserialize)]
pub struct GetDashboards {
    pub username: String,
}

impl Message for GetDashboards {
    type Result = Result<Vec<Dashboard>, ServiceError>;
}

impl From<LoggedUser> for GetDashboards {
    fn from(logged_user: LoggedUser) -> Self {
        GetDashboards {
            username: logged_user.username,
        }
    }
}

impl Handler<GetDashboards> for DbExecutor {
    type Result = Result<Vec<Dashboard>, ServiceError>;
    fn handle(&mut self, msg: GetDashboards, _: &mut Self::Context) -> Self::Result {
        self.0
            .get_user(&msg.username)
            .and_then(|user| self.0.get_dashboards_for_user(&user))
            .map_err(|_e| ServiceError::BadRequest("Username and Password don't match".into()))
    }
}
