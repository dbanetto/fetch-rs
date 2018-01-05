// pub mod series;
// pub mod info_uri;

use iron::prelude::*;
use iron::response::WriteBody;
use mount::Mount;
use util::ApiResult;
use serde_json;
use iron::status::Status;
use hyper::header::ContentType;
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
 

fn index() -> ApiResult<String, String> {
    ApiResult::ok("API available".to_owned())
}

fn handle_index(_: &mut Request) -> IronResult<Response> {

    let result = index();
    let mut resp = Response::new();
    resp.headers.set(
        ContentType(Mime(TopLevel::Application, SubLevel::Json,
                         vec![(Attr::Charset, Value::Utf8)]))
        );

    resp.status = if result.success {
        Some(Status::Ok)
    } else {
        Some(Status::InternalServerError)
    };

    let bytes = serde_json::to_vec(&result).unwrap();
    resp.body = Some(Box::new(bytes));

    Ok(resp)
}

pub fn routes() -> Mount {
    let mut mount = Mount::new();

    let router = router!(
        index: get "/"  => handle_index,
    );

    mount.mount("/", router);
    mount
}
