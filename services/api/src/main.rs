#![recursion_limit = "256"]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt_derive;

#[cfg(test)]
extern crate iron_test;

pub mod config;
pub mod db;
pub mod error;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod util;

use dotenv::dotenv;
use iron::prelude::*;

fn main() {
    dotenv().ok();

    let config = match config::get_config() {
        Ok(config) => config,
        Err(err) => {
            println!("Config error: {}", err);
            std::process::exit(1);
        }
    };

    let addr = format!("{}:{}", config.bind, config.port);

    let mut chain = Chain::new(routes::routes());

    chain.link_before(middleware::Timer);
    chain.link_before(db::get_pool(&config.database_url));

    chain.link_after(middleware::ErrorLog);

    println!("Starting server on {}", addr);
    Iron::new(chain).http(addr).unwrap();
}
