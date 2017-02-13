use rocket::Rocket;

use std::path::{PathBuf, Path};
use models::*;
use diesel::prelude::*;
use rocket::request::Form;
use rocket::response::NamedFile;
use rocket_contrib::{JSON, Template};
use db::DB;
use util::ApiResult;
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

    context.insert("series".to_owned(), to_value(result));

    Template::render("index", &context)
}

#[post("/new", data="<series>")]
fn new_series(_db: DB, series: Form<SeriesForm>) -> JSON<ApiResult<NewSeries, String>> {
    let new_series = NewSeries::from(series.into_inner());

    ApiResult::json(if let Err(e) = new_series.validate() {
        Err(format!("{}", e))
    } else {
        Ok(new_series)
    })
}

#[get("/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

fn routes() -> Vec<Route> {
    routes![index, static_files]
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/", routes())
          .mount("/api/v1", api::routes())
          .mount("/api/v1/series", api::series::routes())
}
