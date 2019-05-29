pub mod dashboard;
pub mod user;
pub mod widget;
pub mod category;

pub type Result<T> = std::result::Result<T, diesel::result::Error>;
