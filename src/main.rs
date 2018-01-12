#![recursion_limit="128"]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate router;
#[macro_use] extern crate serde_derive;
extern crate chrono;
extern crate dotenv;
extern crate durationfmt;
extern crate filetime;
extern crate hyper;
extern crate iron;
extern crate mount;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
extern crate serde_json;
extern crate staticfile;
extern crate time;

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

use iron::prelude::*;

fn main() {
    let mut chain = Chain::new(routes::routes());

    chain.link_before(middleware::Timer);
    chain.link_before(db::get_pool());
    chain.link_after(middleware::ErrorLog);

    let addr = "127.0.0.1:3000";
    println!("Starting server on {}", addr);
    Iron::new(chain).http(addr).unwrap();
}
