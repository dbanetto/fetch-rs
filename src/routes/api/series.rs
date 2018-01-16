use db::DbConnection;
use models::*;
use schema::{info_blob, series};
use super::info_blob::{new_blob, update_blob};
use util::{api_error, api_success, ApiResult};

use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use iron::prelude::*;
use iron::status::Status;
use router::Router;
use serde_json;

use std::error::Error;
use std::io::Read;
use std::str::FromStr;

fn all(req: &mut Request) -> IronResult<Response> {
    let conn = match req.extensions.get::<DbConnection>().unwrap().get() {
        Ok(conn) => conn,
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    let result = series::dsl::series.load::<Series>(&*conn);

    let resp = ApiResult::new(result.map_err(|e| e.description().to_owned())).into();

    Ok(resp)
}


fn select(req: &mut Request) -> IronResult<Response> {
    let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("id") {
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

    let series: Series = match series::dsl::series
        .filter(series::id.eq(series_id))
        .select(series::all_columns)
        .first(&*conn)
    {
        Ok(s) => s,
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    Ok(api_success(series))
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

    let (new_series, blobs) = series_form.into_new();

    if let Err(err) = new_series.validate() {
        return Err(api_error(err, Status::BadRequest));
    }

    let conn = match req.extensions.get::<DbConnection>().unwrap().get() {
        Ok(conn) => conn,
        Err(err) => return Err(api_error(err, Status::RequestTimeout)),
    };

    let new_series: Series = match insert_into(series::table)
        .values(&new_series)
        .returning(series::all_columns)
        .get_result(&*conn)
    {
        Ok(series) => series,
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    let new_blobs: Vec<InfoBlob> = match blobs {
        Some(blobs) => {
            let blobs = blobs
                .into_iter()
                .map(|i| i.into_insertable(&new_series))
                .collect::<Vec<NewInfoBlob>>();
            match insert_into(info_blob::table)
                .values(&blobs)
                .returning(info_blob::all_columns)
                .get_results(&*conn)
            {
                Ok(blobs) => blobs,
                Err(err) => return Err(api_error(err, Status::BadRequest)),
            }
        }
        None => Vec::new(),
    };

    let mut result = serde_json::to_value(new_series).unwrap();
    result
        .as_object_mut()
        .unwrap()
        .insert("blob".to_owned(), serde_json::to_value(new_blobs).unwrap());

    Ok(api_success(result))
}

fn update_series(req: &mut Request) -> IronResult<Response> {
    let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("id") {
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

    let mut buf = vec![];
    match req.body.read_to_end(&mut buf) {
        Ok(_) => (),
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    let series_form: SeriesForm = match serde_json::from_slice(&buf) {
        Ok(form) => form,
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    let (series_put, blobs) = series_form.into_new();

    if let Err(err) = series_put.validate() {
        return Err(api_error(err, Status::BadRequest));
    }

    let series: Series = match update(series::dsl::series.filter(series::id.eq(series_id)))
        .set((
            series::title.eq(series_put.title),
            series::poster_url.eq(series_put.poster_url),
        ))
        .returning(series::all_columns)
        .get_result(&*conn)
    {
        Ok(s) => s,
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    if let Some(blobs) = blobs {
        for blob in blobs {
            if blob.id.is_some() {
                // update
                update_blob(&*conn, series_id, blob);
            } else {
                // create
                new_blob(&*conn, series_id, blob);
            }
        }
    }

    Ok(api_success(series))
}

fn delete_series(req: &mut Request) -> IronResult<Response> {
    // #[delete("/<series_id>")]
    //db: DB, series_id: i32
    let series_id: i32 = match req.extensions.get::<Router>().unwrap().find("id") {
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

    match delete(series::dsl::series.filter(series::id.eq(series_id)))
        .returning(series::all_columns)
        .get_result(&*conn)
    {
        Ok(result) => Ok(api_success::<Series>(result)),
        Err(err) => Err(api_error(err, Status::BadRequest)),
    }
}


pub fn routes() -> Router {
    router!(
        series_index: get "/" => all,
        series_select: get "/:id" => select,
        series_update: put "/:id" => update_series,
        series_new: post "/new" => new,
        series_delete: delete "/:id" => delete_series,
        )
}
