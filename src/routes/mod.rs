use router::Router;
use staticfile::Static;
use mount::Mount;
use std::time::Duration;

pub mod api;

use handlers::CachedFile;

pub fn routes() -> Mount {
    let mut mount = Mount::new();

    mount.mount(
        "/",
        CachedFile::new("public/index.html", Duration::from_secs(60 * 60)),
    );
    mount.mount("/public/", Static::new("public/"));
    mount.mount("/api/v1/", api::routes());
    mount
}
