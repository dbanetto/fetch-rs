use crate::db::{PooledConn, PooledConnFilter};
use crate::error::Result;
use crate::models::*;
use crate::util::api_response;

use serde_json;
use warp::{filters::path, filters::BoxedFilter, Filter, Reply};

fn all(series_id: i32, conn: PooledConn) -> Result<Vec<InfoBlob>> {
    InfoBlob::all(&*conn, series_id)
}

fn select(series_id: i32, blob_id: i32, conn: PooledConn) -> Result<InfoBlob> {
    InfoBlob::get(&*conn, series_id, blob_id)
}

// fn update_api(req: &mut Request) -> IronResult<Response> {
//     let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("series_id") {
//         Some(id) => i32::from_str(id).map_err(|err| api_error(err, Status::BadRequest))?,
//         None => unreachable!(),
//     };

//     let blob_id: i32 = match req.extensions.get::<Router>().unwrap().find("blob_id") {
//         Some(id) => i32::from_str(id).map_err(|err| api_error(err, Status::BadRequest))?,
//         None => unreachable!(),
//     };

//     let mut buf = vec![];
//     match req.body.read_to_end(&mut buf) {
//         Ok(_) => (),
//         Err(err) => return Err(api_error(err, Status::BadRequest)),
//     };

//     let mut blob_update: InfoBlobForm = match serde_json::from_slice(&buf) {
//         Ok(form) => form,
//         Err(err) => return Err(api_error(err, Status::BadRequest)),
//     };

//     blob_update.id = Some(blob_id);

//     let conn = req
//         .extensions
//         .get::<DbConnection>()
//         .unwrap()
//         .get()
//         .map_err(|err| api_error(err, Status::RequestTimeout))?;

//     api_response(
//         InfoBlob::update(&*conn, series_id, blob_update),
//         Status::InternalServerError,
//     )
// }

// fn new(req: &mut Request) -> IronResult<Response> {
//     let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("series_id") {
//         Some(id) => match i32::from_str(id) {
//             Ok(value) => value,
//             Err(err) => return Err(api_error(err, Status::BadRequest)),
//         },
//         None => unreachable!(),
//     };

//     let mut buf = vec![];
//     match req.body.read_to_end(&mut buf) {
//         Ok(_) => (),
//         Err(err) => return Err(api_error(err, Status::BadRequest)),
//     };

//     let blob_form: InfoBlobForm = match serde_json::from_slice(&buf) {
//         Ok(form) => form,
//         Err(err) => return Err(api_error(err, Status::BadRequest)),
//     };

//     let conn = req
//         .extensions
//         .get::<DbConnection>()
//         .unwrap()
//         .get()
//         .map_err(|err| api_error(err, Status::RequestTimeout))?;

//     api_response(
//         InfoBlob::new(&*conn, series_id, blob_form),
//         Status::InternalServerError,
//     )
// }

fn delete(series_id: i32, blob_id: i32, conn: PooledConn) -> Result<InfoBlob> {
    InfoBlob::delete(&*conn, series_id, blob_id)
}

fn select_types(series_id: i32, types: String, conn: PooledConn) -> Result<Vec<InfoBlob>> {
    InfoBlob::get_types(&*conn, series_id, types.split("+").collect())
}

pub fn routes(db_filter: PooledConnFilter) -> BoxedFilter<(impl Reply,)> {
    let all = warp::filters::method::get2()
        .and(path!("api" / "info" / i32))
        .and(path::end())
        .and(db_filter.clone())
        .map(all)
        .map(api_response);

    let select = warp::filters::method::get2()
        .and(path!("api" / "info" / i32 / i32))
        .and(path::end())
        .and(db_filter.clone())
        .map(select)
        .map(api_response);

    let delete = warp::filters::method::delete2()
        .and(path!("api" / "info" / i32 / i32))
        .and(path::end())
        .and(db_filter.clone())
        .map(delete)
        .map(api_response);

    let select_types = warp::filters::method::get2()
        .and(path!("api" / "info" / i32 / "types" / String))
        .and(path::end())
        .and(db_filter.clone())
        .map(select_types)
        .map(api_response);

    warp::any()
        .and(all.or(select).or(delete).or(select_types))
        .with(warp::log("api::info"))
        .boxed()
}
