use crate::schema::ts_number;

#[derive(Identifiable, Queryable, PartialEq, Debug, Serialize)]
#[table_name = "ts_number"]
pub struct TsNumber {
    pub id: i32,
    pub ts: i64,
    pub val: f32,
    pub ack: bool,
    pub _from: i32,
    pub q: i32,
}
