use crate::actions::Result;
use crate::models::datapoint::Datapoints as Datapoint;
use crate::ConnectionPool;
use diesel::prelude::*;

impl ConnectionPool {
    pub fn get_datapoint(&self, datapoint_name: &str) -> Result<Datapoint> {
        use crate::schema::datapoints::dsl::*;

        let conn = self.connection();

        datapoints
            .filter(name.eq(datapoint_name))
            .first::<Datapoint>(&conn)
    }
}
