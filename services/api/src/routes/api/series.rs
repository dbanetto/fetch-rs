use crate::db::DbConnection;
use crate::models::*;
use crate::routes::api::info_blob::{new_blob, update_blob};
use crate::schema::{info_blob, series};
use crate::util::{api_error, api_response, api_success};

use diesel::dsl::*;
use diesel::prelude::*;
use diesel::{delete, insert_into, update};
use iron::prelude::*;
use iron::status::Status;
use router::Router;
use serde_json;

use std::io::Read;
use std::str::FromStr;

fn all(req: &mut Request) -> IronResult<Response> {
    let conn = match req.extensions.get::<DbConnection>().unwrap().get() {
        Ok(conn) => conn,
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    api_response(
        series::dsl::series.load::<Series>(&*conn),
        Status::InternalServerError,
    )
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

    if let Some(put_blobs) = blobs {
        // list of blobs to NOT delete
        let mut seen_blobs: Vec<i32> = vec![];

        for blob in put_blobs {
            if blob.id.is_some() {
                // update
                match update_blob(&*conn, series_id, blob) {
                    // register that this blob was seen
                    Ok(updated_blob) => seen_blobs.push(updated_blob.id),
                    Err(err) => return Err(api_error(err, Status::BadRequest)),
                }
            } else {
                // create
                match new_blob(&*conn, series_id, blob) {
                    // register that this blob was seen
                    Ok(updated_blob) => seen_blobs.push(updated_blob.id),
                    Err(err) => return Err(api_error(err, Status::BadRequest)),
                }
            }
        }

        // delete info_blobs that were not apart of the PUT
        match delete(
            info_blob::dsl::info_blob
                .filter(not(info_blob::id.eq(any(seen_blobs))))
                .filter(info_blob::series_id.eq(series_id)),
        )
        .execute(&*conn)
        {
            Ok(_) => (),
            Err(err) => return Err(api_error(err, Status::InternalServerError)),
        };
    }

    Ok(api_success(series))
}

fn delete_series(req: &mut Request) -> IronResult<Response> {
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
