use crate::db::DbConnection;
use crate::models::{InfoBlob, InfoBlobForm};
use crate::util::{api_error, api_response};

use iron::prelude::*;
use iron::status::Status;
use router::Router;
use serde_json;

use std::io::Read;
use std::str::FromStr;

fn all(req: &mut Request) -> IronResult<Response> {
    let series_id = match req.extensions.get::<Router>().unwrap().find("series_id") {
        Some(id) => i32::from_str(id).map_err(|err| api_error(err, Status::BadRequest))?,
        None => unreachable!(),
    };

    let conn = req
        .extensions
        .get::<DbConnection>()
        .unwrap()
        .get()
        .map_err(|err| api_error(err, Status::RequestTimeout))?;

    api_response(InfoBlob::all(&*conn, series_id), Status::NotFound)
}

fn select(req: &mut Request) -> IronResult<Response> {
    let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("series_id") {
        Some(id) => i32::from_str(id).map_err(|err| api_error(err, Status::BadRequest))?,
        None => unreachable!(),
    };

    let blob_id: i32 = match req.extensions.get::<Router>().unwrap().find("blob_id") {
        Some(id) => i32::from_str(id).map_err(|err| api_error(err, Status::BadRequest))?,
        None => unreachable!(),
    };

    let conn = req
        .extensions
        .get::<DbConnection>()
        .unwrap()
        .get()
        .map_err(|err| api_error(err, Status::RequestTimeout))?;

    api_response(InfoBlob::get(&*conn, series_id, blob_id), Status::NotFound)
}

fn update_api(req: &mut Request) -> IronResult<Response> {
    let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("series_id") {
        Some(id) => i32::from_str(id).map_err(|err| api_error(err, Status::BadRequest))?,
        None => unreachable!(),
    };

    let blob_id: i32 = match req.extensions.get::<Router>().unwrap().find("blob_id") {
        Some(id) => i32::from_str(id).map_err(|err| api_error(err, Status::BadRequest))?,
        None => unreachable!(),
    };

    let mut buf = vec![];
    match req.body.read_to_end(&mut buf) {
        Ok(_) => (),
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    let mut blob_update: InfoBlobForm = match serde_json::from_slice(&buf) {
        Ok(form) => form,
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    blob_update.id = Some(blob_id);

    let conn = req
        .extensions
        .get::<DbConnection>()
        .unwrap()
        .get()
        .map_err(|err| api_error(err, Status::RequestTimeout))?;

    api_response(
        InfoBlob::update(&*conn, series_id, blob_update),
        Status::InternalServerError,
    )
}

fn delete_api(req: &mut Request) -> IronResult<Response> {
    let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("series_id") {
        Some(id) => i32::from_str(id).map_err(|err| api_error(err, Status::BadRequest))?,
        None => unreachable!(),
    };

    let blob_id: i32 = match req.extensions.get::<Router>().unwrap().find("blob_id") {
        Some(id) => i32::from_str(id).map_err(|err| api_error(err, Status::BadRequest))?,
        None => unreachable!(),
    };

    let conn = req
        .extensions
        .get::<DbConnection>()
        .unwrap()
        .get()
        .map_err(|err| api_error(err, Status::RequestTimeout))?;

    api_response(
        InfoBlob::delete(&*conn, series_id, blob_id),
        Status::NotFound,
    )
}

fn new(req: &mut Request) -> IronResult<Response> {
    let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("series_id") {
        Some(id) => match i32::from_str(id) {
            Ok(value) => value,
            Err(err) => return Err(api_error(err, Status::BadRequest)),
        },
        None => unreachable!(),
    };

    let mut buf = vec![];
    match req.body.read_to_end(&mut buf) {
        Ok(_) => (),
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    let blob_form: InfoBlobForm = match serde_json::from_slice(&buf) {
        Ok(form) => form,
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    let conn = req
        .extensions
        .get::<DbConnection>()
        .unwrap()
        .get()
        .map_err(|err| api_error(err, Status::RequestTimeout))?;

    api_response(
        InfoBlob::new(&*conn, series_id, blob_form),
        Status::InternalServerError,
    )
}

fn select_types(req: &mut Request) -> IronResult<Response> {
    let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("series_id") {
        Some(id) => i32::from_str(id).map_err(|err| api_error(err, Status::BadRequest))?,
        None => unreachable!(),
    };

    let select_types: Vec<&str> = match req.extensions.get::<Router>().unwrap().find("types") {
        Some(select_type) => select_type.split("+").collect(),
        None => unreachable!(),
    };

    let conn = req
        .extensions
        .get::<DbConnection>()
        .unwrap()
        .get()
        .map_err(|err| api_error(err, Status::RequestTimeout))?;

    api_response(
        InfoBlob::get_types(&*conn, series_id, select_types),
        Status::NotFound,
    )
}

pub fn routes() -> Router {
    router::router!(
    all: get "/:series_id" => all,
    select_types: get "/:series_id/types/:types" => select_types,
    select: get "/:series_id/:blob_id" => select,
    update: put "/:series_id/:blob_id" => update_api,
    delete: delete "/:series_id/:blob_id" => delete_api,
    create: post "/:series_id/new" => new,
    )
}
