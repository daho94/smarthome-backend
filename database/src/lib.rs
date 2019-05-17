#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

pub mod actions;
pub mod models;
pub mod schema;

// use self::models::post::{NewPost, Post};

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::env;

pub type PgConnectionPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct ConnectionPool {
    pool: PgConnectionPool,
}

embed_migrations!("./migrations");

impl Default for ConnectionPool {
    fn default() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        ConnectionPool::new(&database_url)
    }
}

impl ConnectionPool {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);

        let pool = Pool::builder()
            .build(manager)
            .expect("Failed creating database pool");
        info!("Running pending migrations...");
        let conn = (&pool).get().unwrap();
        if let Err(e) = embedded_migrations::run_with_output(&conn, &mut std::io::stdout()) {
            eprintln!(
                "[DB:embedded_migrations] Error while running pending migrations: {}",
                e
            );
        };
        ConnectionPool { pool }
    }
    pub fn connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool.get().unwrap()
    }
    pub fn ping(&self) -> bool {
        let conn = self.connection();
        diesel::sql_query(r#"SELECT 1"#).execute(&conn).is_ok()
    }
    // pub fn create_post(&self, title: &str, body: &str) -> Post {
    //     use schema::posts;

    //     let conn = self.connection();
    //     let new_post = NewPost { title, body };

    //     diesel::insert_into(posts::table)
    //         .values(&new_post)
    //         .get_result(&conn)
    //         .expect("Error saving new post")
    // }
}
