use actix_web::web;
use database::ConnectionPool;
use diesel::result::Error;

#[derive(Debug, Deserialize)]
pub struct IobrokerState {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct HistoryAvailability {
    pub has_history: bool,
}

pub fn has_datapoint_history(
    pool: web::Data<ConnectionPool>,
    name: &str,
) -> Result<HistoryAvailability, Error> {
    let availability = match pool.get_datapoint(name) {
        Ok(_) => true,
        Err(_) => false,
    };

    Ok(HistoryAvailability {
        has_history: availability,
    })
}

#[derive(Debug, Deserialize)]
pub struct IobrokerStateHistory {
    pub name: String,
    pub from: i64,
    pub to: i64,
}

#[derive(Debug, Serialize)]
pub struct Datapoint {
    pub timestamp: i64,
    pub val: f32,
}

impl From<&(i64, f32)> for Datapoint {
    fn from(tuple: &(i64, f32)) -> Self {
        Datapoint {
            timestamp: tuple.0,
            val: tuple.1,
        }
    }
}

pub fn get_datapoints_in_interval(
    pool: web::Data<ConnectionPool>,
    name: &str,
    from: i64,
    to: i64,
) -> Result<Vec<Datapoint>, Error> {
    pool.get_values_for_datapoint(name, from, to)
        .and_then(|res| Ok(res.iter().map(|tuple| tuple.into()).collect()))
}
