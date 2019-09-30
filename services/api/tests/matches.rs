use fetch::models::{InfoBlobForm, SeriesForm};
use fetch::data::{DataSource, memory::MemoryDatabase};
use fetch::routes::routes;

use std::default::Default;
use warp::{Filter, Reply};

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
    ($name:ident, $method:expr, $path:expr, $body:expr) => {
        #[test]
        fn $name() {
            let router = make_router();

            let body = serde_json::to_vec(&$body).expect("Failed to serialize body");

            assert_eq!(
                warp::test::request()
                    .method($method)
                    .header("Content-Length", format!("{}", body.len()))
                    .body(&body)
                    .path($path)
                    .matches(&router),
                true
            );
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
route_matches!(match_series_new, "POST", "/series", SeriesForm::default());
route_matches!(
    match_series_new_trail,
    "POST",
    "/series/",
    SeriesForm::default()
);
route_matches!(
    match_series_update,
    "PUT",
    "/series/1",
    SeriesForm::default()
);
route_matches!(
    match_series_update_trail,
    "PUT",
    "/series/1/",
    SeriesForm::default()
);
route_matches!(match_series_delete, "DELETE", "/series/1");
route_matches!(match_series_delete_trail, "DELETE", "/series/1/");

// Info endpoints
route_matches!(match_info_all, "GET", "/info/1");
route_matches!(match_info_all_trail, "GET", "/info/1/");
route_matches!(match_info_types, "GET", "/info/1/types/type");
route_matches!(match_info_types_trail, "GET", "/info/1/types/type/");

route_matches!(match_info_view, "GET", "/info/1/1");
route_matches!(match_info_view_trail, "GET", "/info/1/1/");
route_matches!(
    match_info_update,
    "PUT",
    "/info/1/1",
    InfoBlobForm::default()
);
route_matches!(
    match_info_update_trail,
    "PUT",
    "/info/1/1/",
    InfoBlobForm::default()
);
route_matches!(match_info_delete, "DELETE", "/info/1/1");
route_matches!(match_info_delete_trail, "DELETE", "/info/1/1/");
