use crate::db::{PooledConn, PooledConnFilter};
use crate::error::Result;
use crate::util::api_response;

use diesel::prelude::*;
use warp::{filters::path, Filter, Reply};

pub mod api;

pub fn routes(db_filter: PooledConnFilter) -> impl Filter<Extract = (impl Reply,)> {
    let healthcheck = warp::filters::method::get2()
        .and(path!("healthcheck"))
        .and(path::end())
        .and(db_filter.clone())
        .map(healthcheck)
        .map(api_response);

    warp::any()
        .and(healthcheck.or(api::routes(db_filter)))
        .with(warp::log("api"))
}

pub fn healthcheck(conn: PooledConn) -> Result<bool> {
    (&*conn)
        .execute("SELECT 1;")
        .map(|_| true)
        .map_err(|err| err.into())
}
