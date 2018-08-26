use db::DbConnection;
use mount::Mount;
use iron::prelude::*;
use iron::status::Status;
use util::{api_error, api_success};
use diesel::prelude::*;

pub mod api;

pub fn routes() -> Mount {
    let mut mount = Mount::new();

    // endpoints
    mount.mount("/healthcheck", healthcheck);
    mount.mount("/", api::routes());

    mount
}

fn healthcheck(req: &mut Request) -> IronResult<Response> {
    let conn = match req.extensions.get::<DbConnection>().unwrap().get() {
        Ok(conn) => conn,
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    match (&*conn).execute("SELECT 1;") {
        Ok(_) => Ok(api_success("healthy")),
        Err(err) => Err(api_error(err, Status::InternalServerError)),
    }
}
