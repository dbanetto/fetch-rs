use db::DbConnection;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use error::*;
use iron::prelude::*;
use iron::status::Status;
use models::{InfoBlob, InfoBlobForm, Series};
use router::Router;
use schema::{info_blob, series};
use serde_json;
use std::io::Read;
use std::str::FromStr;
use util::{api_error, api_response, api_success};

fn all(req: &mut Request) -> IronResult<Response> {
    let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("series_id") {
        Some(id) => match i32::from_str(id) {
            Ok(value) => value,
            Err(err) => return Err(api_error(err, Status::BadRequest)),
        },
        None => unreachable!(),
    };

    let conn = match req.extensions.get::<DbConnection>().unwrap().get() {
        Ok(conn) => conn,
        Err(err) => return Err(api_error(err, Status::RequestTimeout)),
    };

    let blobs: Vec<InfoBlob> = match info_blob::dsl::info_blob
        .filter(info_blob::series_id.eq(series_id))
        .get_results(&*conn)
    {
        Ok(blobs) => blobs,
        Err(err) => return Err(api_error(err, Status::NotFound)),
    };

    Ok(api_success(blobs))
}

fn select(req: &mut Request) -> IronResult<Response> {
    let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("series_id") {
        Some(id) => match i32::from_str(id) {
            Ok(value) => value,
            Err(err) => return Err(api_error(err, Status::BadRequest)),
        },
        None => unreachable!(),
    };

    let blob_id: i32 = match req.extensions.get::<Router>().unwrap().find("blob_id") {
        Some(id) => match i32::from_str(id) {
            Ok(value) => value,
            Err(err) => return Err(api_error(err, Status::BadRequest)),
        },
        None => unreachable!(),
    };

    let conn = match req.extensions.get::<DbConnection>().unwrap().get() {
        Ok(conn) => conn,
        Err(err) => return Err(api_error(err, Status::RequestTimeout)),
    };

    let blob: InfoBlob = match info_blob::dsl::info_blob
        .filter(info_blob::series_id.eq(series_id))
        .filter(info_blob::id.eq(blob_id))
        .first(&*conn)
    {
        Ok(blob) => blob,
        Err(err) => return Err(api_error(err, Status::NotFound)),
    };

    Ok(api_success(blob))
}

fn update_api(req: &mut Request) -> IronResult<Response> {
    let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("series_id") {
        Some(id) => match i32::from_str(id) {
            Ok(value) => value,
            Err(err) => return Err(api_error(err, Status::BadRequest)),
        },
        None => unreachable!(),
    };

    let blob_id: i32 = match req.extensions.get::<Router>().unwrap().find("blob_id") {
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

    let mut blob_update: InfoBlobForm = match serde_json::from_slice(&buf) {
        Ok(form) => form,
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    blob_update.id = Some(blob_id);

    let conn = match req.extensions.get::<DbConnection>().unwrap().get() {
        Ok(conn) => conn,
        Err(err) => return Err(api_error(err, Status::RequestTimeout)),
    };

    api_response(
        update_blob(&*conn, series_id, blob_update),
        Status::InternalServerError,
    )
}

fn delete_api(req: &mut Request) -> IronResult<Response> {
    let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("series_id") {
        Some(id) => match i32::from_str(id) {
            Ok(value) => value,
            Err(err) => return Err(api_error(err, Status::BadRequest)),
        },
        None => unreachable!(),
    };

    let blob_id: i32 = match req.extensions.get::<Router>().unwrap().find("blob_id") {
        Some(id) => match i32::from_str(id) {
            Ok(value) => value,
            Err(err) => return Err(api_error(err, Status::BadRequest)),
        },
        None => unreachable!(),
    };

    let conn = match req.extensions.get::<DbConnection>().unwrap().get() {
        Ok(conn) => conn,
        Err(err) => return Err(api_error(err, Status::RequestTimeout)),
    };

    let blob: InfoBlob = match delete(
        info_blob::dsl::info_blob
            .filter(info_blob::id.eq(blob_id))
            .filter(info_blob::series_id.eq(series_id)),
    ).get_result(&*conn)
    {
        Ok(blob) => blob,
        Err(err) => return Err(api_error(err, Status::NotFound)),
    };

    Ok(api_success(blob))
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

    let conn = match req.extensions.get::<DbConnection>().unwrap().get() {
        Ok(conn) => conn,
        Err(err) => return Err(api_error(err, Status::RequestTimeout)),
    };

    api_response(
        new_blob(&*conn, series_id, blob_form),
        Status::InternalServerError,
    )
}

fn select_types(req: &mut Request) -> IronResult<Response> {
    let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("series_id") {
        Some(id) => match i32::from_str(id) {
            Ok(value) => value,
            Err(err) => return Err(api_error(err, Status::BadRequest)),
        },
        None => unreachable!(),
    };

    let select_types: Vec<&str> = match req.extensions.get::<Router>().unwrap().find("types") {
        Some(select_type) => select_type.split("+").collect(),
        None => unreachable!(),
    };

    let conn = match req.extensions.get::<DbConnection>().unwrap().get() {
        Ok(conn) => conn,
        Err(err) => return Err(api_error(err, Status::RequestTimeout)),
    };

    let blobs: Vec<InfoBlob> = match info_blob::dsl::info_blob
        .filter(info_blob::series_id.eq(series_id))
        .filter(info_blob::info_type.eq_any(select_types))
        .get_results(&*conn)
    {
        Ok(blob) => blob,
        Err(err) => return Err(api_error(err, Status::NotFound)),
    };

    Ok(api_success(blobs))
}

pub fn new_blob(conn: &PgConnection, series_id: i32, blob_form: InfoBlobForm) -> Result<InfoBlob> {
    let series: Series = match series::dsl::series
        .filter(series::id.eq(series_id))
        .select(series::all_columns)
        .first(&*conn)
    {
        Ok(s) => s,
        Err(e) => return Err(e.into()),
    };

    let blob = blob_form.into_insertable(&series);

    insert_into(info_blob::table)
        .values(&blob)
        .returning(info_blob::all_columns)
        .get_result(&*conn)
        .map_err(|err| err.into())
}

pub fn update_blob(
    conn: &PgConnection,
    series_id: i32,
    blob_update: InfoBlobForm,
) -> Result<InfoBlob> {
    let blob_id = match blob_update.id {
        Some(id) => id,
        None => return Err("id not given".into()),
    };

    update(
        info_blob::dsl::info_blob
            .filter(info_blob::id.eq(blob_id))
            .filter(info_blob::series_id.eq(series_id)),
    ).set((
        info_blob::blob.eq(blob_update.blob),
        info_blob::info_type.eq(blob_update.info_type),
    )).returning(info_blob::all_columns)
    .get_result(&*conn)
    .map_err(|e| e.into())
}

pub fn routes() -> Router {
    router!(
        all: get "/:series_id" => all,
        select_types: get "/:series_id/types/:types" => select_types,
        select: get "/:series_id/:blob_id" => select,
        update: put "/:series_id/:blob_id" => update_api,
        delete: delete "/:series_id/:blob_id" => delete_api,
        create: post "/:series_id/new" => new,
        )
}
