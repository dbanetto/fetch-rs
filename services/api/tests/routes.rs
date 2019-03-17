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

macro_rules! route_matches {
    ($name:ident, $method:expr, $path:expr) => {
        #[test]
        fn $name() {
            let router = make_router();

            assert!(warp::test::request()
                .method($method)
                .path($path)
                .matches(&router));
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

// 
route_matches!(match_index, "GET", "/");
route_matches!(match_healthcheck, "GET", "/healthcheck");
route_matches!(match_healthcheck_trail, "GET", "/healthcheck/");

// Series endpoints
route_matches!(match_series_all, "GET", "/series");
route_matches!(match_series_all_trail, "GET", "/series/");

// Series resource endpoints
route_matches!(match_series_view, "GET", "/series/1");
route_matches!(match_series_view_trail, "GET", "/series/1/");
route_matches!(match_series_new, "POST", "/series");
route_matches!(match_series_new_trail, "POST", "/series/");
route_matches!(match_series_update, "PUT", "/series/1");
route_matches!(match_series_update_trail, "PUT", "/series/1/");
route_matches!(match_series_delete, "DELETE", "/series/1");
route_matches!(match_series_delete_trail, "DELETE", "/series/1/");

// Info endpoints
route_matches!(match_info_all, "GET", "/info/1");
route_matches!(match_info_all_trail, "GET", "/info/1/");
route_matches!(match_info_types, "GET", "/info/1/types/type");
route_matches!(match_info_types_trail, "GET", "/info/1/types/type/");

route_matches!(match_info_view, "GET", "/info/1/1");
route_matches!(match_info_view_trail, "GET", "/info/1/1/");
route_matches!(match_info_update, "PUT", "/info/1/1");
route_matches!(match_info_update_trail, "PUT", "/info/1/1/");
route_matches!(match_info_delete, "DELETE", "/info/1/1");
route_matches!(match_info_delete_trail, "DELETE", "/info/1/1/");

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
    200,
    json!({ "success": true, "data": [] })
);
