use crate::error::Result;
use crate::util::api_response;
use crate::db::PooledConnFilter;

use warp::{Filter, Reply};

// pub mod info_blob;
pub mod series;


fn index() -> Result<String> {
    Ok("API available".to_owned())
}

pub fn routes(db_filter: PooledConnFilter) -> impl Filter<Extract=(impl Reply,)> {
    let index_route = warp::filters::method::get2()
        .and(path!("api"))
        .map(index)
        .map(api_response);

    warp::any()
        .and(
            series::routes(db_filter)
        )
        .with(warp::log("api"))
}

// #[cfg(test)]
// mod test {
//     use super::index;
//     use iron::headers::Headers;
//     use iron_test::{request, response};
//     use serde_json::{self, Value};

//     #[test]
//     fn index_json() {
//         // url path is hack to make iron_test to accept the url
//         let res = request::get("http://_/", Headers::new(), &index).unwrap();

//         let result: Value = serde_json::from_str(&response::extract_body_to_string(res)).unwrap();

//         assert!(result.get("success").is_some());
//         assert!(result.get("data").is_some());
//         assert!(result.get("error").is_none());
//     }
// }
