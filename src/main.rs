#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;

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

use models::*;
use diesel::prelude::*;
use diesel::Connection;
use diesel::pg::PgConnection;
use rocket_contrib::JSON;
use db::DB;

#[get("/")]
fn index(db: DB) -> JSON<Vec<Series>> {
    use self::schema::series::dsl::*;

    let conn = db.conn();

    let result = series.load::<Series>(conn)
                       .expect("Error loading posts");

    JSON(result)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
