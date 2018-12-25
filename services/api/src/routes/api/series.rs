use crate::db::DbConnection;
use crate::models::*;
use crate::util::{api_error, api_response, api_success};

use iron::prelude::*;
use iron::status::Status;
use router::Router;
use serde_json;

use std::io::Read;
use std::str::FromStr;

fn all(req: &mut Request) -> IronResult<Response> {
    let conn = req
        .extensions
        .get::<DbConnection>()
        .unwrap()
        .get()
        .map_err(|err| api_error(err, Status::RequestTimeout))?;

    api_response(Series::all(&*conn), Status::InternalServerError)
}

fn select(req: &mut Request) -> IronResult<Response> {
    let series_id = match req.extensions.get::<Router>().unwrap().find("id") {
        Some(id) => i32::from_str(id).map_err(|err| api_error(err, Status::BadRequest))?,
        None => unreachable!(),
    };

    let conn = req
        .extensions
        .get::<DbConnection>()
        .unwrap()
        .get()
        .map_err(|err| api_error(err, Status::RequestTimeout))?;

    api_response(Series::get(&*conn, series_id), Status::NotFound)
}

fn new(req: &mut Request) -> IronResult<Response> {
    let mut buf = vec![];
    match req.body.read_to_end(&mut buf) {
        Ok(_) => (),
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    let series_form: SeriesForm = match serde_json::from_slice(&buf) {
        Ok(form) => form,
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    let conn = req
        .extensions
        .get::<DbConnection>()
        .unwrap()
        .get()
        .map_err(|err| api_error(err, Status::RequestTimeout))?;

    let (series, blobs) =
        Series::new(&*conn, series_form).map_err(|err| api_error(err, Status::BadRequest))?;

    let result = serde_json::to_value(series)
        .unwrap()
        .as_object_mut()
        .unwrap()
        .insert("blob".to_owned(), serde_json::to_value(blobs).unwrap());

    Ok(api_success(result))
}

fn update_series(req: &mut Request) -> IronResult<Response> {
    let series_id = match req.extensions.get::<Router>().unwrap().find("id") {
        Some(id) => i32::from_str(id).map_err(|err| api_error(err, Status::BadRequest))?,
        None => unreachable!(),
    };

    let conn = req
        .extensions
        .get::<DbConnection>()
        .unwrap()
        .get()
        .map_err(|err| api_error(err, Status::RequestTimeout))?;

    let mut buf = vec![];
    match req.body.read_to_end(&mut buf) {
        Ok(_) => (),
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    let series_form: SeriesForm = match serde_json::from_slice(&buf) {
        Ok(form) => form,
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    let (series, blobs) = Series::update(&*conn, series_id, series_form)
        .map_err(|err| api_error(err, Status::BadRequest))?;

    let result = serde_json::to_value(series)
        .unwrap()
        .as_object_mut()
        .unwrap()
        .insert("blob".to_owned(), serde_json::to_value(blobs).unwrap());

    Ok(api_success(result))
}

fn delete_series(req: &mut Request) -> IronResult<Response> {
    let series_id = match req.extensions.get::<Router>().unwrap().find("id") {
        Some(id) => i32::from_str(id).map_err(|err| api_error(err, Status::BadRequest))?,
        None => unreachable!(),
    };

    let conn = req
        .extensions
        .get::<DbConnection>()
        .unwrap()
        .get()
        .map_err(|err| api_error(err, Status::RequestTimeout))?;

    api_response(Series::delete(&*conn, series_id), Status::NotFound)
}

pub fn routes() -> Router {
    router::router!(
    series_index: get "/" => all,
    series_select: get "/:id" => select,
    series_update: put "/:id" => update_series,
    series_new: post "/new" => new,
    series_delete: delete "/:id" => delete_series,
    )
}
