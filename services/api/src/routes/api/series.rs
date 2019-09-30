use crate::data::{DatabaseFilter, DatabaseFiltered};
use crate::error::Result;
use crate::models::*;
use crate::util::api_response;

use warp::{filters::body, filters::path, filters::BoxedFilter, Filter, Reply};

fn all(source: DatabaseFiltered) -> Result<Vec<Series>> {
    source.all_series()
}

fn select(id: SeriesId, source: DatabaseFiltered) -> Result<Series> {
    source.get_series(id)
}

fn new(form: SeriesForm, source: DatabaseFiltered) -> Result<SeriesBlob> {
    source.new_series(form)
}

fn update(id: SeriesId, form: SeriesForm, source: DatabaseFiltered) -> Result<SeriesBlob> {
    source.update_series(id, form)
}

fn delete(id: SeriesId, source: DatabaseFiltered) -> Result<Series> {
    source.delete_series(id)
}

pub fn routes(data_filter: DatabaseFilter) -> BoxedFilter<(impl Reply,)> {
    let all = warp::filters::method::get2()
        .and(path!("series"))
        .and(path::end())
        .and(data_filter.clone())
        .map(all)
        .map(api_response);

    let new = warp::filters::method::post2()
        .and(path!("series"))
        .and(path::end())
        .and(body::content_length_limit(1024 * 64))
        .and(body::json::<SeriesForm>())
        .and(data_filter.clone())
        .map(new)
        .map(api_response);

    let select = warp::filters::method::get2()
        .and(path!("series" / SeriesId))
        .and(path::end())
        .and(data_filter.clone())
        .map(select)
        .map(api_response);

    let update = warp::filters::method::put2()
        .and(path!("series" / SeriesId))
        .and(path::end())
        .and(body::content_length_limit(1024 * 64))
        .and(body::json::<SeriesForm>())
        .and(data_filter.clone())
        .map(update)
        .map(api_response);

    let delete = warp::filters::method::delete2()
        .and(path!("series" / SeriesId))
        .and(path::end())
        .and(data_filter.clone())
        .map(delete)
        .map(api_response);

    warp::any()
        .and(all.or(select).or(new).or(update).or(delete))
        .boxed()
}
