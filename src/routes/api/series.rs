use db::DbConnection;
use diesel::prelude::*;
use diesel::{delete, insert, update};
use iron::prelude::*;
use iron::status::Status;
use models::*;
use router::Router;
use schema::{info_uri, series};
use serde_json::{self, Value};
use std::io::Read;
use std::error::Error;
use std::str::FromStr;
use super::info_uri::{new_uri, update_uri};
use util::{api_error, ApiResult};

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

    Ok(ApiResult::<Series, String>::ok(series).into())
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

    let (new_series, info_uris) = series_form.into_new();

    if let Err(err) = new_series.validate() {
        return Err(api_error(err, Status::BadRequest));
    }

    let conn = match req.extensions.get::<DbConnection>().unwrap().get() {
        Ok(conn) => conn,
        Err(err) => return Err(api_error(err, Status::RequestTimeout)),
    };

    let new_series: Series = match insert(&new_series)
        .into(series::table)
        .returning(series::all_columns)
        .get_result(&*conn)
    {
        Ok(series) => series,
        Err(err) => return Err(api_error(err, Status::BadRequest)),
    };

    let new_info_uris: Vec<InfoUri> = match info_uris {
        Some(uris) => {
            let info_uris = uris.into_iter()
                .map(|i| i.into_insertable(&new_series))
                .collect::<Vec<NewInfoUri>>();
            match insert(&info_uris)
                .into(info_uri::table)
                .returning(info_uri::all_columns)
                .get_results(&*conn)
            {
                Ok(uris) => uris,
                Err(err) => return Err(api_error(err, Status::BadRequest)),
            }
        }
        None => Vec::new(),
    };

    let mut result = serde_json::to_value(new_series).unwrap();
    result.as_object_mut().unwrap().insert(
        "info_uri".to_owned(),
        serde_json::to_value(new_info_uris).unwrap(),
    );

    Ok(ApiResult::<Value, String>::ok(result).into())
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

    let (series_put, info_uris) = series_form.into_new();

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

    if let Some(uris) = info_uris {
        for uri in uris {
            if uri.id.is_some() {
                // update
                update_uri(&*conn, series_id, uri);
            } else {
                // create
                new_uri(&*conn, series_id, uri);
            }
        }
    }

    Ok(ApiResult::<Series, String>::ok(series).into())
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
        Ok(result) => Ok(ApiResult::<Series, String>::ok(result).into()),
        Err(err) => Err(api_error(err, Status::BadRequest)),
    }
}


pub fn routes() -> Router {
    router!(
        series_index: get "/" => all,
        series_select: get "/:id" => select,
        series_update: put "/:id" => update_series,
        series_new: post "/" => new,
        series_delete: delete "/:id" => delete_series,
        )
}
