use crate::models::{user::User, dashboard::*};
use crate::ConnectionPool;
use diesel::prelude::*;
use serde_json::Value;
use crate::actions::Result;


impl ConnectionPool {
    pub fn get_dashboard_by_user(&self, dashboard_name: &str, user: &User) -> Result<Dashboard> {
        use crate::schema::dashboards::dsl::*;

        let conn = self.connection();
        Dashboard::belonging_to(user).filter(name.eq(dashboard_name)).first::<Dashboard>(&conn)
    }

    pub fn get_dashboards_by_user(&self, user: &User) -> Result<Vec<Dashboard>> {
        let conn = self.connection();
        Dashboard::belonging_to(user).load::<Dashboard>(&conn)
    }

    pub fn create_dashboard_for_user(&self, user: &User, dashboard_name: &str, settings: &Value) -> Dashboard {
        use crate::schema::dashboards;

        let conn = self.connection();
        let new_dashboard = NewDashboard {
            user_id: user.id,
            name: dashboard_name,
            default_dashboard: false,
            settings
        };

        diesel::insert_into(dashboards::table)
            .values(&new_dashboard)
            .get_result(&conn)
            .expect("Error saving new dashboard")
    }

    pub fn save_dashboard_for_user(&self, user: &User, dashboard_name: &str, new_settings: &Value) -> Dashboard {
        use crate::schema::dashboards;
        use crate::schema::dashboards::dsl::*;

        let conn = self.connection();

        diesel::update(dashboards::table.filter(user_id.eq(user.id))
            .filter(name.eq(dashboard_name)))
            .set(settings.eq(new_settings))
            .get_result(&conn)
            .expect("Error updating dashboard")
    }
    
}