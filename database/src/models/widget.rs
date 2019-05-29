use crate::schema::widgets;
use super::category::Category;

#[derive(Identifiable,Associations ,Queryable, PartialEq, Debug, Serialize)]
#[belongs_to(Category)]
#[table_name = "widgets"]
pub struct Widget {
    pub id: i32,
    pub category_id: i32,
    pub name: String,
    pub component_key: String,
}

#[derive(Insertable, Debug)]
#[table_name = "widgets"]
pub struct NewWidget<'a> {
    pub category_id: i32,
    pub name: &'a str,
    pub component_key: &'a str,

}