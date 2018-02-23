#![recursion_limit = "128"]

extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derives;
extern crate dotenv;
extern crate durationfmt;
#[macro_use]
extern crate error_chain;
extern crate filetime;
extern crate hyper;
extern crate iron;
extern crate mount;
extern crate r2d2;
#[macro_use]
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate staticfile;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate time;
extern crate toml;

#[cfg(test)]
extern crate iron_test;

pub mod db;
pub mod error;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod schema;
pub mod util;
pub mod config;

use iron::prelude::*;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let config = match config::get_config() {
        Ok(config) => config,
        Err(err) => {
            println!("Config error: {}", err);
            std::process::exit(1);
        }
    };
    println!("{:?}", config);

    let mut chain = Chain::new(routes::routes());

    chain.link_before(middleware::Timer);
    chain.link_before(db::get_pool(config.database_url));
    chain.link_after(middleware::ErrorLog);

    let addr = format!("{}:{}", config.bind, config.port);
    println!("Starting server on {}", addr);
    Iron::new(chain).http(addr).unwrap();
}
