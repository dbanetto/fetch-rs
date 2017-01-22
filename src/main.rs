#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

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

use std::path::{PathBuf, Path};
use models::*;
use diesel::prelude::*;
use rocket::request::Form;
use rocket::response::NamedFile;
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

#[post("/new", data="<series>")]
fn new_series(_db: DB, series: Form<SeriesForm>) -> JSON<SeriesForm> {
    JSON(series.into_inner())
}

#[get("/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![
               index,
               new_series,
               static_files]).launch();
}
