use crate::error::Result;
use crate::util::api_response;
use crate::models::*;
use crate::db::{ PooledConn ,PooledConnFilter };

use serde_json;
use warp::{Filter, Reply};

fn all(conn: PooledConn) -> Result<Vec<Series>> {
    Series::all(&*conn)
}

fn select(id: i32, conn: PooledConn) -> Result<Series> {
    Series::get(&*conn, id)
}

// fn new(req: &mut Request) -> IronResult<Response> {
//     let mut buf = vec![];
//     match req.body.read_to_end(&mut buf) {
//         Ok(_) => (),
//         Err(err) => return Err(api_error(err, Status::BadRequest)),
//     };

//     let series_form: SeriesForm = match serde_json::from_slice(&buf) {
//         Ok(form) => form,
//         Err(err) => return Err(api_error(err, Status::BadRequest)),
//     };

//     let conn = req
//         .extensions
//         .get::<DbConnection>()
//         .unwrap()
//         .get()
//         .map_err(|err| api_error(err, Status::RequestTimeout))?;

//     let (series, blobs) =
//         Series::new(&*conn, series_form).map_err(|err| api_error(err, Status::BadRequest))?;

//     let result = serde_json::to_value(series)
//         .unwrap()
//         .as_object_mut()
//         .unwrap()
//         .insert("blob".to_owned(), serde_json::to_value(blobs).unwrap());

//     Ok(api_success(result))
// }

// fn update_series(req: &mut Request) -> IronResult<Response> {
//     let series_id = match req.extensions.get::<Router>().unwrap().find("id") {
//         Some(id) => i32::from_str(id).map_err(|err| api_error(err, Status::BadRequest))?,
//         None => unreachable!(),
//     };

//     let conn = req
//         .extensions
//         .get::<DbConnection>()
//         .unwrap()
//         .get()
//         .map_err(|err| api_error(err, Status::RequestTimeout))?;

//     let mut buf = vec![];
//     match req.body.read_to_end(&mut buf) {
//         Ok(_) => (),
//         Err(err) => return Err(api_error(err, Status::BadRequest)),
//     };

//     let series_form: SeriesForm = match serde_json::from_slice(&buf) {
//         Ok(form) => form,
//         Err(err) => return Err(api_error(err, Status::BadRequest)),
//     };

//     let (series, blobs) = Series::update(&*conn, series_id, series_form)
//         .map_err(|err| api_error(err, Status::BadRequest))?;

//     let result = serde_json::to_value(series)
//         .unwrap()
//         .as_object_mut()
//         .unwrap()
//         .insert("blob".to_owned(), serde_json::to_value(blobs).unwrap());

//     Ok(api_success(result))
// }

fn delete(id: i32, conn: PooledConn) -> Result<Series> {
    Series::delete(&*conn, id)
}

pub fn routes(db_filter: PooledConnFilter) -> impl Filter<Extract=(impl Reply,)> {
    let all = warp::filters::method::get2()
        .and(path!("api" / "series"))
        .and(db_filter.clone())
        .map(all)
        .map(api_response);

    let select = warp::filters::method::get2()
        .and(path!("api" / "series" / i32))
        .and(db_filter.clone())
        .map(select)
        .map(api_response);

    let delete = warp::filters::method::delete2()
        .and(path!("api" / "series" / i32))
        .and(db_filter.clone())
        .map(delete)
        .map(api_response);

    warp::any()
        .and(
            all
            .or(select)
            .or(delete)
        )
        .with(warp::log("api::series"))
}
