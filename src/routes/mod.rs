use std::path::{PathBuf, Path};
use router::Router;
use staticfile::Static;
use mount::Mount;

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
    
    // uses a mount to remove the /public/ part of the request
    // so a file can be served from that directory
    let mut public_dir = Mount::new();
    public_dir.mount("/public/", Static::new("public/"));

    router!(
       index: get "/" => Static::new("public/index.html"),
       public: get "/public/*" => public_dir,
       // TODO: default route always returns index.html
    )
}
