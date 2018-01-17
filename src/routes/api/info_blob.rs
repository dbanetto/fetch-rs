use db::DbConnection;
use models::{InfoBlob, InfoBlobForm, Series};
use schema::{info_blob, series};
use util::{api_error, api_success, ApiResult};
use std::str::FromStr;
use iron::status::Status;
use iron::prelude::*;
use diesel::prelude::*;
use std::io::Read;
use serde_json;
use router::Router;
use diesel::pg::PgConnection;
use diesel::{delete, insert_into, update};

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
        Err(err) => return Err(api_error(err, Status::BadRequest)),
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
        Err(err) => return Err(api_error(err, Status::BadRequest)),
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

    Ok(update_blob(&*conn, series_id, blob_update).into())
}

// #[delete("/<series_id>/blob/<blob_id>")]
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
        Err(err) => return Err(api_error(err, Status::BadRequest)),
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

    Ok(new_blob(&*conn, series_id, blob_form).into())
}

fn primary(req: &mut Request) -> IronResult<Response> {
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

    let blobs: InfoBlob = match info_blob::dsl::info_blob
        .filter(info_blob::series_id.eq(series_id))
        .filter(info_blob::primary.eq(true))
        .first(&*conn)
    {
        Ok(blob) => blob,
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    Ok(api_success(blobs))
}

pub fn new_blob(
    conn: &PgConnection,
    series_id: i32,
    blob_form: InfoBlobForm,
) -> ApiResult<InfoBlob, String> {
    let series: Series = match series::dsl::series
        .filter(series::id.eq(series_id))
        .select(series::all_columns)
        .first(&*conn)
    {
        Ok(s) => s,
        Err(e) => return ApiResult::err_format(e),
    };

    let blob = blob_form.into_insertable(&series);

    match insert_into(info_blob::table)
        .values(&blob)
        .returning(info_blob::all_columns)
        .get_result(&*conn)
    {
        Ok(blob) => ApiResult::ok(blob),
        Err(e) => ApiResult::err_format(e),
    }
}

pub fn update_blob(
    conn: &PgConnection,
    series_id: i32,
    blob_update: InfoBlobForm,
) -> ApiResult<InfoBlob, String> {
    let blob_id = match blob_update.id {
        Some(id) => id,
        None => return ApiResult::err("id not given".to_owned()),
    };

    let query = update(
        info_blob::dsl::info_blob
            .filter(info_blob::id.eq(blob_id))
            .filter(info_blob::series_id.eq(series_id)),
    );

    // prevent overriding primary if it is not given
    let result = if blob_update.primary.is_some() {
        query
            .set((
                info_blob::blob.eq(blob_update.blob),
                info_blob::info_type.eq(blob_update.info_type),
                info_blob::primary.eq(blob_update.primary.unwrap()),
            ))
            .returning(info_blob::all_columns)
            .get_result(&*conn)
    } else {
        query
            .set((
                info_blob::blob.eq(blob_update.blob),
                info_blob::info_type.eq(blob_update.info_type),
            ))
            .returning(info_blob::all_columns)
            .get_result(&*conn)
    };

    ApiResult::new(result.map_err(|e| format!("{}", e)))
}

pub fn routes() -> Router {
    router!(
        all: get "/:series_id" => all,
        primary: get "/:series_id/primary" => primary,
        select: get "/:series_id/:blob_id" => select,
        update: put "/:series_id/:blob_id" => update_api,
        delete: delete "/:series_id/:blob_id" => delete_api,
        create: post "/:series_id/new" => new,
        )
}
