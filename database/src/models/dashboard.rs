use super::user::User;
use crate::schema::dashboards;
use serde_json::Value;

#[derive(Identifiable, Associations, Queryable, PartialEq, Debug, Deserialize, Serialize)]
#[belongs_to(User)]
#[table_name = "dashboards"]
pub struct Dashboard {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub default_dashboard: bool,
    pub settings: Value,
    pub icon: String,
}

#[derive(Insertable, Debug)]
#[table_name = "dashboards"]
pub struct NewDashboard<'a> {
    pub user_id: i32,
    pub name: &'a str,
    pub default_dashboard: bool,
    pub settings: &'a Value,
    pub icon: &'a str,
}
