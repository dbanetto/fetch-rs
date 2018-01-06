pub mod series;
// pub mod info_uri;

use iron::prelude::*;
use mount::Mount;
use util::ApiResult;
use serde_json;
use iron::status::Status;
use hyper::mime::{Attr, Mime, SubLevel, TopLevel, Value};


fn index() -> ApiResult<String, String> {
    ApiResult::ok("API available".to_owned())
}

fn handle_index(_: &mut Request) -> IronResult<Response> {
    let result = index();

    let status = if result.success {
        Status::Ok
    } else {
        Status::InternalServerError
    };

    let bytes = serde_json::to_vec(&result).unwrap();

    Ok(Response::with((
        status,
        bytes,
        Mime(
            TopLevel::Application,
            SubLevel::Json,
            vec![(Attr::Charset, Value::Utf8)],
        ),
    )))
}

pub fn routes() -> Mount {
    let mut mount = Mount::new();

    let router = router!(
        index: get "/"  => handle_index,
    );

    mount.mount("/", router);
    mount.mount("/series/", series::routes());
    mount
}
