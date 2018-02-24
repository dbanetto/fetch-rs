use staticfile::Static;
use mount::Mount;
use iron::prelude::*;
use iron::status::Status;
use hbs::Template;

use config::Config;
use error::*;

pub mod api;

pub fn routes() -> Mount {
    let mut mount = Mount::new();

    // static files
    mount.mount("/", index);
    mount.mount("/public/", Static::new("public/"));

    // endpoints
    mount.mount("/api/v1/", api::routes());

    mount
}

fn index(req: &mut Request) -> IronResult<Response> {
    let config = match req.extensions.get::<Config>() {
        Some(config) => config,
        None => {
            return Err(IronError::new(
                Error::from(ErrorKind::ConfigReadFailed),
                Status::InternalServerError,
            ));
        }
    };

    Ok(Response::with((Template::new("index", config), Status::Ok)))
}
