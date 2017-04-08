use rocket::Rocket;

use std::path::{PathBuf, Path};
use rocket::response::NamedFile;
use rocket_contrib::Template;
use std::collections::HashMap;
use serde_json::Value;
use rocket::Route;

mod api;

#[get("/")]
fn root() -> Template {
    let context: HashMap<String, Value> = HashMap::new();
    Template::render("index", &context)
}

#[get("/<path..>")]
fn index(path: Option<PathBuf>) -> Template {
    let context: HashMap<String, Value> = HashMap::new();
    Template::render("index", &context)
}

#[get("/public/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

fn routes() -> Vec<Route> {
    routes![static_files, index, root]
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/v1", api::routes())
        .mount("/api/v1/series", api::series::routes())
        .mount("/", routes())
}
