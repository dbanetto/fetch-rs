use crate::error::Result;
use crate::db::{PooledConnFilter, PooledConn};
use crate::util::api_response;
use crate::models;

use diesel::prelude::*;
use warp::{Filter, Reply};

mod api;

pub fn routes(db_filter: PooledConnFilter) -> impl Filter<Extract=(impl Reply,)> {

    let healthcheck = warp::filters::method::get2()
        .and(path!("healthcheck"))
        .and(db_filter.clone())
        .map(healthcheck)
        .map(api_response);

    let select = warp::filters::method::get2()
        .and(path!("api" / "series" / i32))
        .and(db_filter.clone())
        .map(|id: i32, pool: PooledConn| models::Series::get(&*pool, id))
        .map(api_response);

    warp::any()
        .and(
            healthcheck
            .or(select)
        )
        .with(warp::log("api"))
}

pub fn healthcheck(conn: PooledConn) -> Result<bool> {
    (&*conn).execute("SELECT 1;")
        .map(|_| true)
        .map_err(|err| err.into())
}
