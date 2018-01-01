use std::path::{PathBuf, Path};
use router::Router;
use staticfile::Static;

// pub mod api;

// #[get("/")]
// fn root() -> Option<NamedFile> {
//     NamedFile::open(Path::new("public").join("index.html")).ok()
// }

// #[get("/series/<path..>")]
// fn series(path: Option<PathBuf>) -> Option<NamedFile> {
//     NamedFile::open(Path::new("public").join("index.html")).ok()
// }

// #[get("/public/<file..>")]
// fn static_files(file: PathBuf) -> Option<NamedFile> {
//     NamedFile::open(Path::new("public/").join(file)).ok()
// }


pub fn routes() -> Router {
    router!(
       public: get "/public/" => Static::new(Path::new("public/")),
    )
}
