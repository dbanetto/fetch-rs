pub mod series;
// pub mod info_uri;

use iron::prelude::*;
use mount::Mount;
use util::ApiResult;

fn index() -> ApiResult<String, String> {
    ApiResult::ok("API available".to_owned())
}

fn handle_index(_: &mut Request) -> IronResult<Response> {
    Ok(index().into())
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
