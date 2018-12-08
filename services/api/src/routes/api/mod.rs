pub mod info_blob;
pub mod series;

use crate::util::api_success;
use iron::prelude::*;
use mount::Mount;

fn index(_: &mut Request) -> IronResult<Response> {
    Ok(api_success("API available"))
}

pub fn routes() -> Mount {
    let mut mount = Mount::new();

    let router = router!(
        index: get "/"  => index,
    );

    mount.mount("/", router);
    mount.mount("/series/", series::routes());
    mount.mount("/info/", info_blob::routes());
    mount
}

#[cfg(test)]
mod test {
    use super::index;
    use iron::headers::Headers;
    use iron_test::{request, response};
    use serde_json::{self, Value};

    #[test]
    fn index_json() {
        // url path is hack to make iron_test to accept the url
        let res = request::get("http://_/", Headers::new(), &index).unwrap();

        let result: Value = serde_json::from_str(&response::extract_body_to_string(res)).unwrap();

        assert!(result.get("success").is_some());
        assert!(result.get("data").is_some());
        assert!(result.get("error").is_none());
    }
}
