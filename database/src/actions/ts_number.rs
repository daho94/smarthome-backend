use crate::actions::Result;
use crate::ConnectionPool;
use diesel::prelude::*;

impl ConnectionPool {
    pub fn get_values_for_datapoint(
        &self,
        datapoint_name: &str,
        from: i64,
        to: i64,
    ) -> Result<Vec<(i64, f32)>> {
        use crate::schema::datapoints::dsl::*;
        use crate::schema::ts_number::dsl as ts_number;

        let conn = self.connection();

        datapoints
            .inner_join(ts_number::ts_number.on(ts_number::id.eq(id)))
            .select((ts_number::ts, ts_number::val))
            .filter(name.eq(datapoint_name))
            .filter(ts_number::ts.between(from, to))
            .order(ts_number::ts.asc())
            .load(&conn)
    }
}
