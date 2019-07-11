use crate::schema::categories;

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize)]
#[table_name = "categories"]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Debug)]
#[table_name = "categories"]
pub struct NewCategory<'a> {
    pub name: &'a str,
}
