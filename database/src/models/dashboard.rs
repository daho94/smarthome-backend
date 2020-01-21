use super::dashboard_folder::DashboardFolder;
use super::user::User;
use crate::schema::dashboards;
use serde_json::Value;

#[derive(Identifiable, Associations, Queryable, PartialEq, Debug, Deserialize, Serialize)]
#[belongs_to(User)]
#[belongs_to(DashboardFolder)]
#[table_name = "dashboards"]
pub struct Dashboard {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub default_dashboard: bool,
    pub settings: Value,
    pub icon: String,
    pub dashboard_folder_id: i32,
}

#[derive(Insertable, Debug)]
#[table_name = "dashboards"]
pub struct NewDashboard<'a> {
    pub user_id: i32,
    pub name: &'a str,
    pub default_dashboard: bool,
    pub settings: &'a Value,
    pub icon: &'a str,
    pub dashboard_folder_id: i32,
}
