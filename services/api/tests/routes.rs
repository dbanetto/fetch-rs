use fetch::config::get_config;
use fetch::db::get_pool;
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
    dotenv::dotenv().ok();
    let _ = env_logger::try_init();

    let config = match get_config() {
        Ok(config) => config,
        Err(err) => {
            panic!("Config error: {}", err);
        }
    };

    routes(get_pool(&config.database_url).unwrap())
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
// route_get_tests!(
//     get_series_all,
//     "GET",
//     "/series", //FIXME: this is fragile test
//     200,
//     json!({ "success": true, "data": [] })
// );
// route_get_tests!(
//     get_info_all,
//     "GET",
//     "/info/1", // FIXME: this is fragile test
//     200,
//     json!({ "success": true, "data": [] })
// );
