use rocket::Rocket;

use std::path::{PathBuf, Path};
use rocket::response::NamedFile;
use rocket::Route;

mod api;

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("public").join("index.html")).ok()
}

#[get("/<path..>")]
fn static_files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public").join(path)).ok()
}

fn routes() -> Vec<Route> {
    routes![index, static_files]
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/v1", api::routes())
        .mount("/api/v1/series", api::series::routes())
        .mount("/", routes())
}
