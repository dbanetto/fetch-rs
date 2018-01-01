#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate router;
#[macro_use] extern crate serde_derive;
extern crate chrono;
extern crate dotenv;
extern crate iron;
extern crate mount;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
extern crate serde_json;
extern crate staticfile;

pub mod db;
pub mod error;
pub mod models;
pub mod routes;
pub mod schema;
pub mod util;

use iron::prelude::*;

fn main() {
    let chain = Chain::new(routes::routes());
    Iron::new(chain).http("127.0.0.1:3000").unwrap();
}
