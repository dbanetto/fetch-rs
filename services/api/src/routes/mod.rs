use crate::data::{DatabaseFilter, DatabaseFiltered};
use crate::error::Result;
use crate::util::api_response;

use warp::{filters::path, Filter, Reply};

pub mod api;

pub fn routes(data_filter: DatabaseFilter) -> impl Filter<Extract = (impl Reply,)> {
    let healthcheck = warp::filters::method::get2()
        .and(path!("healthcheck"))
        .and(path::end())
        .and(data_filter.clone())
        .map(healthcheck)
        .map(api_response);

    warp::any()
        .and(healthcheck.or(api::routes(data_filter.clone())))
        .with(warp::log("api"))
}

pub fn healthcheck(source: DatabaseFiltered) -> Result<bool> {
    source.healthcheck()
}
