use super::user::User;
use crate::schema::dashboard_folders;

#[derive(Identifiable, Associations, Queryable, PartialEq, Debug, Deserialize, Serialize)]
#[belongs_to(User)]
#[table_name = "dashboard_folders"]
pub struct DashboardFolder {
    pub id: i32,
    pub parent_id: i32,
    pub user_id: i32,
    pub name: String,
    pub icon: String,
}

#[derive(Insertable, Debug)]
#[table_name = "dashboard_folders"]
pub struct NewDashboardFolder<'a> {
    pub parent_id: i32,
    pub user_id: i32,
    pub name: &'a str,
    pub icon: &'a str,
}
