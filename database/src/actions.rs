pub mod category;
pub mod dashboard;
pub mod datapoint;
pub mod ts_number;
pub mod user;
pub mod widget;

pub type Result<T> = std::result::Result<T, diesel::result::Error>;
