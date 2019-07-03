extern crate actix;
#[macro_use]
extern crate serde_derive;

mod app;
mod models;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use chrono::Duration;
use database::ConnectionPool;
use dotenv::dotenv;
use rustls::{
    internal::pemfile::{certs, rsa_private_keys},
    NoClientAuth, ServerConfig,
};
use std::{env, fs::File, io};

mod api;
mod errors;

fn main() -> io::Result<()> {
    // setup env
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("Smarthome_Server");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create connection pool
    let pool = ConnectionPool::new(&database_url);

    // load ssl keys
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut io::BufReader::new(File::open("cert/invalid.cer").unwrap());
    let key_file = &mut io::BufReader::new(File::open("cert/invalid.key").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = rsa_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    // create server
    let server = HttpServer::new(move || {
        let secret: String = env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
        let _domain: String = env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(secret.as_bytes())
                    .name("auth")
                    .path("/")
                    .max_age(Duration::weeks(1).num_seconds())
                    .secure(true),
            ))
            .configure(app::config)
    });


    server.bind_rustls("127.0.0.1:3000", config)?.start();

    sys.run()
}
