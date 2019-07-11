use crate::schema::datapoints;

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize)]
#[table_name = "datapoints"]
pub struct Datapoints {
    pub id: i32,
    pub name: String,
    pub data_type: i32,
}
