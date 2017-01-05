#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate dotenv;

use std::env;

use dotenv::dotenv;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;

use rocket::request;
use rocket::http::Status;
use rocket::request::{FromRequest, Request};
use rocket::outcome::Outcome::*;
use rocket::response::status::*;

lazy_static! {
    pub static ref DB_POOL: r2d2::Pool<ConnectionManager<PgConnection>> = {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
        let config = r2d2::Config::default();
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::new(config, manager).expect("Failed to create pool.");
        pool
    };
}

pub struct DB(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl DB {
    pub fn conn(&self) -> &PgConnection {
        &*self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DB {
    type Error = r2d2::GetTimeout;
    fn from_request(_: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match DB_POOL.get() {
            Ok(conn) => Success(DB(conn)),
            Err(e) => Failure((Status::InternalServerError, e)),
        }
    }
}

#[get("/")]
fn home(db: DB) -> &'static str {
    "Hello World"
}

fn main() {
    rocket::ignite().mount("/", routes![home]).launch();
}
