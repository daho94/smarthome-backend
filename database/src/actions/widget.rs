use crate::actions::Result;
use crate::models::{category::Category, widget::*};
use crate::ConnectionPool;
use diesel::prelude::*;

impl ConnectionPool {
    pub fn get_widgets(&self) -> Result<Vec<(Category, Vec<Widget>)>> {
        use crate::schema::categories;

        let conn = self.connection();

        let categories = categories::table.load::<Category>(&conn)?;
        let widgets = Widget::belonging_to(&categories)
            .load::<Widget>(&conn)?
            .grouped_by(&categories);
        let data = categories.into_iter().zip(widgets).collect::<Vec<_>>();

        Ok(data)
    }

    pub fn create_widget_for_category(
        &self,
        widget_name: &str,
        component_key: &str,
        category: &Category,
        icon: &str,
    ) -> Widget {
        use crate::schema::widgets;

        let conn = self.connection();
        let new_widget = NewWidget {
            name: widget_name,
            component_key,
            category_id: category.id,
            icon,
        };

        diesel::insert_into(widgets::table)
            .values(&new_widget)
            .get_result(&conn)
            .expect("Error saving new widget")
    }
}
