use crate::data::DatabaseFilter;
use crate::error::Result;
use crate::util::api_response;

use std::clone::Clone;
use warp::{filters::path, filters::BoxedFilter, Filter, Reply};

pub mod info_blob;
pub mod series;

fn index() -> Result<String> {
    Ok("API available".to_owned())
}

pub fn routes(data_filter: DatabaseFilter) -> BoxedFilter<(impl Reply,)> {
    let index_route = warp::filters::method::get2()
        .and(path::end())
        .map(index)
        .map(api_response);

    warp::any()
        .and(
            index_route
                .or(series::routes(data_filter.clone()))
                .or(info_blob::routes(data_filter.clone())),
        )
        .boxed()
}
