use crate::db::{PooledConn, PooledConnFilter};
use crate::error::{Error, Result};
use crate::models::*;
use crate::util::api_response;

use warp::{filters::body, filters::path, filters::BoxedFilter, Filter, Reply};

fn all(conn: PooledConn) -> Result<Vec<Series>> {
    Series::all(&*conn)
}

fn select(id: SeriesId, conn: PooledConn) -> Result<Series> {
    Series::get(&*conn, id)
}

fn new(form: SeriesForm, conn: PooledConn) -> Result<SeriesBlob> {
    Series::new(&*conn, form).map_err::<Error, _>(|err| err.into())
}

fn update(id: SeriesId, form: SeriesForm, conn: PooledConn) -> Result<SeriesBlob> {
    Series::update(&*conn, id, form).map_err::<Error, _>(|err| err.into())
}

fn delete(id: SeriesId, conn: PooledConn) -> Result<Series> {
    Series::delete(&*conn, id)
}

pub fn routes(db_filter: PooledConnFilter) -> BoxedFilter<(impl Reply,)> {
    let all = warp::filters::method::get2()
        .and(path!("series"))
        .and(path::end())
        .and(db_filter.clone())
        .map(all)
        .map(api_response);

    let new = warp::filters::method::post2()
        .and(path!("series"))
        .and(path::end())
        .and(body::content_length_limit(1024 * 64))
        .and(body::json::<SeriesForm>())
        .and(db_filter.clone())
        .map(new)
        .map(api_response);

    let select = warp::filters::method::get2()
        .and(path!("series" / SeriesId))
        .and(path::end())
        .and(db_filter.clone())
        .map(select)
        .map(api_response);

    let update = warp::filters::method::put2()
        .and(path!("series" / SeriesId))
        .and(path::end())
        .and(body::content_length_limit(1024 * 64))
        .and(body::json::<SeriesForm>())
        .and(db_filter.clone())
        .map(update)
        .map(api_response);

    let delete = warp::filters::method::delete2()
        .and(path!("series" / SeriesId))
        .and(path::end())
        .and(db_filter.clone())
        .map(delete)
        .map(api_response);

    warp::any()
        .and(all.or(select).or(new).or(update).or(delete))
        .boxed()
}
