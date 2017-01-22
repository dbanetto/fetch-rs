use rocket::Rocket;

use std::path::{PathBuf, Path};
use models::*;
use diesel::prelude::*;
use rocket::request::Form;
use rocket::response::NamedFile;
use rocket_contrib::JSON;
use db::DB;

#[get("/")]
fn index(db: DB) -> JSON<Vec<Series>> {
    use ::schema::series::dsl::*;

    let conn = db.conn();

    let result = series.load::<Series>(conn)
        .expect("Error loading posts");

    JSON(result)
}

#[post("/new", data="<series>")]
fn new_series(_db: DB, series: Form<SeriesForm>) -> Result<JSON<NewSeries>, JSON<String>> {
    let new_series = NewSeries::from(series.into_inner());

    if let Err(e) = new_series.validate() {
        Err(JSON(format!("{}", e)))
    } else {
        Ok(JSON(new_series))
    }
}

#[get("/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/", routes![index, new_series, static_files])
}
