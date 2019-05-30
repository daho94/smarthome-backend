use actix_web::web;
use database::models::{category::Category, widget::Widget};
use database::ConnectionPool;
use diesel::result::Error;

pub fn get_widgets_grouped(
    pool: web::Data<ConnectionPool>,
) -> Result<Vec<(Category, Vec<Widget>)>, Error> {
    pool.get_widgets()
}
