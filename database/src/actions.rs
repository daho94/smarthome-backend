pub mod user;
pub mod dashboard;

pub type Result<T> = std::result::Result<T, diesel::result::Error>;