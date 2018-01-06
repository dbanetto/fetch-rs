use iron::prelude::*;
use router::Router;
use std::error::Error;
use db::DbConnection;
use util::ApiResult;
use models::*;
use diesel::prelude::*;
use schema::{info_uri, series};
// use super::info_uri::{new_uri, update_uri};
use diesel::{delete, insert, update};
use iron::status::Status;
use diesel::pg::PgConnection;

fn all(conn: &PgConnection) -> ApiResult<Vec<Series>, String> {

    let result = series::dsl::series.load::<Series>(conn);

    ApiResult::new(result.map_err(|e| e.description().to_owned()))
}

fn handle_all(req: &mut Request) -> IronResult<Response> {
    let conn = match req.extensions.get::<DbConnection>().unwrap().get() {
        Ok(conn) => conn,
        Err(err) => return Err(IronError::new(err, Status::RequestTimeout)),
    };

    Ok(all(&*conn).into())
}

// #[get("/<series_id>")]
// fn select(db: DB, series_id: i32) -> Json<ApiResult<Series, String>> {
//     let conn = db.conn();

//     let series: Series = match series::dsl::series
//         .filter(series::id.eq(series_id))
//         .select(series::all_columns)
//         .first(conn) {
//         Ok(s) => s,
//         Err(e) => return ApiResult::err_format(e).json(),
//     };

//     ApiResult::ok(series).json()
// }

// #[post("/new", data = "<series_form>")]
// fn new(db: DB, series_form: Json<SeriesForm>) -> Json<ApiResult<serde_json::Value, String>> {
//     let series_form = series_form.into_inner();
//     let (new_series, info_uris) = series_form.into_new();

//     if let Err(e) = new_series.validate() {
//         return ApiResult::err_format(e).json();
//     }

//     let conn = db.conn();

//     let new_series: Series = match insert(&new_series)
//         .into(series::table)
//         .returning(series::all_columns)
//         .get_result(conn) {
//         Ok(s) => s,
//         Err(e) => return ApiResult::err_format(e).json(),
//     };

//     let new_info_uris: Vec<InfoUri> = match info_uris {
//         Some(uris) => {
//             let info_uris = uris.into_iter()
//                 .map(|i| i.into_insertable(&new_series))
//                 .collect::<Vec<NewInfoUri>>();
//             match insert(&info_uris)
//                 .into(info_uri::table)
//                 .returning(info_uri::all_columns)
//                 .get_results(conn) {
//                 Ok(uris) => uris,
//                 Err(e) => return ApiResult::err_format(e).json(),
//             }
//         }
//         None => Vec::new(),
//     };

//     let mut result = serde_json::to_value(new_series).unwrap();
//     result.as_object_mut().unwrap().insert(
//         "info_uri".to_owned(),
//         serde_json::to_value(new_info_uris).unwrap(),
//     );

//     ApiResult::ok(result).json()
// }

// #[put("/<series_id>", data = "<series_form>")]
// fn update_series(
//     db: DB,
//     series_id: i32,
//     series_form: Json<SeriesForm>,
// ) -> Json<ApiResult<Series, String>> {
//     let conn = db.conn();

//     let (series_put, info_uris) = series_form.into_inner().into_new();

//     if let Err(e) = series_put.validate() {
//         return ApiResult::err_format(e).json();
//     }

//     let series: Series = match update(series::dsl::series.filter(series::id.eq(series_id)))
//         .set((
//             series::title.eq(series_put.title),
//             series::poster_url.eq(series_put.poster_url),
//         ))
//         .returning(series::all_columns)
//         .get_result(conn) {
//         Ok(s) => s,
//         Err(e) => return ApiResult::err_format(e).json(),
//     };

//     if let Some(uris) = info_uris {
//         for uri in uris {
//             if uri.id.is_some() {
//                 // update
//                 update_uri(&db, series_id, uri);
//             } else {
//                 // create
//                 new_uri(&db, series_id, uri);
//             }
//         }
//     }

//     ApiResult::ok(series).json()
// }

// #[delete("/<series_id>")]
// fn delete_series(db: DB, series_id: i32) -> Json<ApiResult<Series, String>> {
//     let conn = db.conn();

//     match delete(series::dsl::series.filter(series::id.eq(series_id)))
//         .returning(series::all_columns)
//         .get_result(conn) {
//         Ok(result) => ApiResult::ok(result).json(),
//         Err(e) => ApiResult::err_format(e).json(),
//     }
// }


pub fn routes() -> Router {
    router!(
        index: get "/" => handle_all,
        )
}
