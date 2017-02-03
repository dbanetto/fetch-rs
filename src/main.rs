#![feature(plugin)]
// #![plugin(clippy)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;
extern crate serde;
extern crate serde_json;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
extern crate dotenv;
extern crate chrono;

mod models;
mod schema;
mod db;
mod util;
mod routes;
mod error;

fn main() {
    routes::mount(rocket::ignite()).launch();
}
