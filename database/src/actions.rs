pub mod dashboard;
pub mod user;

pub type Result<T> = std::result::Result<T, diesel::result::Error>;
