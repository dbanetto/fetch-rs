use fetch::data::{memory::MemoryDatabase, DataSource};
use fetch::routes::routes;

use serde_json::{json, Value};
use warp::{Filter, Reply};

macro_rules! route_get_tests {
    ($name:ident, $method:expr, $path:expr, $status:expr, $return:expr) => {
        #[test]
        fn $name() {
            let router = make_router();

            let res = warp::test::request()
                .method($method)
                .path($path)
                .reply(&router);

            assert_eq!(res.status(), $status);

            let body: Value = serde_json::from_slice(res.body()).unwrap();
            assert_eq!(body, $return);
        }
    };
}

fn make_router() -> impl Filter<Extract = (impl Reply,)> {
    let data = MemoryDatabase::default();
    let data_filter = warp::any()
        .map(move || Box::new(data.clone()) as Box<dyn DataSource + Send>)
        .boxed();

    routes(data_filter)
}

route_get_tests!(
    get_index,
    "GET",
    "/",
    200,
    json!({ "success": true, "data": "API available" })
);
route_get_tests!(
    get_healthcheck,
    "GET",
    "/healthcheck",
    200,
    json!({ "success": true, "data": true })
);
route_get_tests!(
    get_series_all,
    "GET",
    "/series", //FIXME: this is fragile test
    200,
    json!({ "success": true, "data": [] })
);
route_get_tests!(
    get_info_all,
    "GET",
    "/info/1", // FIXME: this is fragile test
    404,
    json!({ "success": false, "error": "Not Found" })
);
