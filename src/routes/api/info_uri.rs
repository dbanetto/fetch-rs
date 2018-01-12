use db::DbConnection;
use util::ApiResult;
use diesel::prelude::*;
use models::{InfoUri, InfoUriForm, Series};
use schema::{info_uri, series};
use diesel::{insert, update, delete};
use std::error::Error;

// #[get("/<series_id>/uri")]
// fn all(db: DB, series_id: i32) -> Json<ApiResult<Vec<InfoUri>, String>> {
//     let conn = db.conn();

//     let uris = match info_uri::dsl::info_uri
//         .filter(info_uri::series_id.eq(series_id))
//         .get_results(conn) {
//         Ok(uris) => uris,
//         Err(e) => return ApiResult::err_format(e).json(),
//     };

//     ApiResult::ok(uris).json()
// }

// #[get("/<series_id>/uri/<uri_id>")]
// fn get(db: DB, series_id: i32, uri_id: i32) -> Json<ApiResult<InfoUri, String>> {

//     let conn = db.conn();

//     let uri: InfoUri = match info_uri::dsl::info_uri
//         .filter(info_uri::series_id.eq(series_id))
//         .filter(info_uri::id.eq(uri_id))
//         .first(conn) {
//         Ok(uri) => uri,
//         Err(e) => return ApiResult::err_format(e).json(),
//     };

//     ApiResult::ok(uri).json()
// }

// #[put("/<series_id>/uri/<uri_id>", data = "<uri_update>")]
// fn update_api(
//     db: DB,
//     series_id: i32,
//     uri_id: i32,
//     uri_update: Json<InfoUriForm>,
// ) -> Json<ApiResult<InfoUri, String>> {

//     let uri_update = uri_update.into_inner();

//     update_uri(&db, series_id, uri_update).json()
// }

// #[delete("/<series_id>/uri/<uri_id>")]
// fn delete_api(db: DB, series_id: i32, uri_id: i32) -> Json<ApiResult<InfoUri, String>> {

//     let conn = db.conn();

//     let uri = match delete(
//         info_uri::dsl::info_uri
//             .filter(info_uri::id.eq(uri_id))
//             .filter(info_uri::series_id.eq(series_id)),
//     ).get_result(conn) {
//         Ok(uri) => uri,
//         Err(e) => return ApiResult::err_format(e).json(),
//     };

//     ApiResult::ok(uri).json()
// }

// #[post("/<series_id>/uri/new", data = "<uri_form>")]
// fn new(db: DB, series_id: i32, uri_form: Json<InfoUriForm>) -> Json<ApiResult<InfoUri, String>> {
//     let uri_form = uri_form.into_inner();

//     new_uri(&db, series_id, uri_form).json()
// }

// #[get("/<series_id>/uri_primary")]
// fn primary(db: DB, series_id: i32) -> Json<ApiResult<InfoUri, String>> {
//     let conn = db.conn();

//     let uris = match info_uri::dsl::info_uri
//         .filter(info_uri::series_id.eq(series_id))
//         .filter(info_uri::primary.eq(true))
//         .first(conn) {
//         Ok(uri) => uri,
//         Err(e) => return ApiResult::err_format(e).json(),
//     };

//     ApiResult::ok(uris).json()
// }

pub fn new_uri(db: DbConnection, series_id: i32, uri_form: InfoUriForm) -> ApiResult<InfoUri, String> {

    let conn = match db.pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            return ApiResult::err(err.description().to_owned())
        },
    };

    let series: Series = match series::dsl::series
        .filter(series::id.eq(series_id))
        .select(series::all_columns)
        .first(&*conn) {
        Ok(s) => s,
        Err(e) => return ApiResult::err_format(e),
    };

    let info_uri = uri_form.into_insertable(&series);

    match insert(&info_uri)
        .into(info_uri::table)
        .returning(info_uri::all_columns)
        .get_result(&*conn) {
        Ok(uri) => ApiResult::ok(uri),
        Err(e) => ApiResult::err_format(e),
    }
}

pub fn update_uri(db: DbConnection, series_id: i32, uri_update: InfoUriForm) -> ApiResult<InfoUri, String> {

    let conn = match db.pool.get() {
        Ok(conn) => conn,
        Err(err) => {
            return ApiResult::err(err.description().to_owned())
        },
    };

    let uri_id = match uri_update.id {
        Some(id) => id,
        None => return ApiResult::err("id not given".to_owned()),
    };

    let query = update(
        info_uri::dsl::info_uri
            .filter(info_uri::id.eq(uri_id))
            .filter(info_uri::series_id.eq(series_id)),
    );

    // prevent overriding primary if it is not given
    let result = if uri_update.primary.is_some() {
        query
            .set((
                info_uri::uri.eq(uri_update.uri),
                info_uri::primary.eq(uri_update.primary.unwrap()),
            ))
            .returning(info_uri::all_columns)
            .get_result(&*conn)
    } else {
        query
            .set((info_uri::uri.eq(uri_update.uri),))
            .returning(info_uri::all_columns)
            .get_result(&*conn)
    };

    ApiResult::new(result.map_err(|e| format!("{}", e)))
}

// pub fn routes() -> Vec<Route> {
//     routes![all, primary, get, update_api, delete_api, new]
// }
