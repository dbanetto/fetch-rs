use std::error::Error;
use db::DB;
use util::ApiResult;
use rocket::Route;
use rocket_contrib::JSON;
use diesel::prelude::*;
use models::{InfoUri, InfoUriForm};
use schema::{info_uri, series};
use diesel::{insert, update, delete};
use serde_json;

#[get("/<series_id>/uri")]
fn all(db: DB, series_id: i32) -> JSON<ApiResult<Vec<InfoUri>, String>> {
    let conn = db.conn();

    let uris = match info_uri::dsl::info_uri.filter(info_uri::series_id.eq(series_id))
        .get_results(conn) {
        Ok(uris) => uris,
        Err(e) => return ApiResult::err_format(e).json(),
    };

    ApiResult::ok(uris).json()
}

#[get("/<series_id>/uri/<uri_id>")]
fn get(db: DB, series_id: i32, uri_id: i32) -> JSON<ApiResult<InfoUri, String>> {

    let conn = db.conn();

    let uri: InfoUri = match info_uri::dsl::info_uri.filter(info_uri::series_id.eq(series_id))
        .filter(info_uri::id.eq(uri_id))
        .first(conn) {
        Ok(uri) => uri,
        Err(e) => return ApiResult::err_format(e).json(),
    };

    ApiResult::ok(uri).json()
}

#[put("/<series_id>/uri/<uri_id>", data = "<uri_update>")]
fn update_uri(db: DB,
              series_id: i32,
              uri_id: i32,
              uri_update: JSON<InfoUriForm>)
              -> JSON<ApiResult<InfoUri, String>> {

    let uri_update = uri_update.into_inner();
    let conn = db.conn();


    let query = update(info_uri::dsl::info_uri.filter(info_uri::id.eq(uri_id)));

    // prevent overriding primary if it is not given
    let result = if uri_update.primary.is_some() {
        query.set((info_uri::uri.eq(uri_update.uri),
                  info_uri::primary.eq(uri_update.primary.unwrap())))
            .returning(info_uri::all_columns)
            .get_result(conn)
    } else {
        query.set((info_uri::uri.eq(uri_update.uri),))
            .returning(info_uri::all_columns)
            .get_result(conn)
    };

    let uri: InfoUri = match result {
        Ok(uri) => uri,
        Err(e) => return ApiResult::err_format(e).json(),
    };

    ApiResult::ok(uri).json()
}

#[delete("/<series_id>/uri/<uri_id>")]
fn delete_uri(db: DB, series_id: i32, uri_id: i32) -> JSON<ApiResult<InfoUri, String>> {

    let conn = db.conn();

    let uri = match delete(info_uri::dsl::info_uri.filter(info_uri::id.eq(uri_id)))
        .get_result(conn) {
        Ok(uri) => uri,
        Err(e) => return ApiResult::err_format(e).json(),
    };

    ApiResult::ok(uri).json()
}


pub fn routes() -> Vec<Route> {
    routes![all, get, update_uri, delete_uri]
}
