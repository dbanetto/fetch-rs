use rocket::Rocket;

use std::path::{PathBuf, Path};
use models::*;
use diesel::prelude::*;
use rocket::response::NamedFile;
use rocket_contrib::Template;
use db::DB;
use std::collections::HashMap;
use serde_json::to_value;
use rocket::Route;

mod api;

#[get("/")]
fn index(db: DB) -> Template {
    use ::schema::series::dsl::*;

    let conn = db.conn();
    let mut context = HashMap::new();

    let result = series.load::<Series>(conn).expect("Error loading posts");

    context.insert("series".to_owned(), to_value(result).unwrap());

    Template::render("index", &context)
}

#[get("/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

fn routes() -> Vec<Route> {
    routes![index]
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/", routes())
        .mount("/api/v1", api::routes())
        .mount("/api/v1/series", api::series::routes())
        .mount("/public", routes![static_files])
}
