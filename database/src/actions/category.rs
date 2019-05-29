use crate::models::{category::*};
use crate::ConnectionPool;
use diesel::prelude::*;
use crate::actions::Result;


impl ConnectionPool {    
    pub fn create_category(&self, name: &str) -> Category {
        use crate::schema::categories;

        let conn = self.connection();
        let new_category = NewCategory { name };

        diesel::insert_into(categories::table)
            .values(&new_category)
            .get_result(&conn)
            .expect("Error saving new category")
    }

    pub fn get_category(&self, category_name: &str) -> Result<Category> {
        use crate::schema::categories::dsl::*;

        let conn = self.connection();
        categories.filter(name.eq(category_name)).first::<Category>(&conn)
    }
} 
 
