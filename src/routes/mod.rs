use rocket::Rocket;

use std::path::{PathBuf, Path};
use rocket::response::NamedFile;
use rocket::Route;

pub mod api;

#[get("/")]
fn root() -> Option<NamedFile> {
    NamedFile::open(Path::new("public").join("index.html")).ok()
}

#[get("/series/<path..>")]
fn series(path: Option<PathBuf>) -> Option<NamedFile> {
    NamedFile::open(Path::new("public").join("index.html")).ok()
}

#[get("/public/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

fn routes() -> Vec<Route> {
    routes![static_files, series, root]
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket
        .mount("/api/v1", api::routes())
        .mount("/api/v1/series", api::series::routes())
        .mount("/", routes())
}
