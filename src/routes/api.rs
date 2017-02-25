use rocket::Route;
use rocket_contrib::JSON;
use util::ApiResult;

#[get("/")]
fn index() -> JSON<ApiResult<String, String>> {
    ApiResult::ok("API available".to_owned()).json()
}

pub mod series {

    use std::error::Error;
    use db::DB;
    use util::ApiResult;
    use rocket::Route;
    use rocket_contrib::JSON;
    use models::*;
    use diesel::prelude::*;
    use schema::{info_uri, series};
    use diesel::{insert, update};
    use serde_json;

    #[get("/")]
    fn all(db: DB) -> JSON<ApiResult<Vec<Series>, String>> {
        let conn = db.conn();

        let result = series::dsl::series.load::<Series>(conn);

        ApiResult::new(result.map_err(|e| e.description().to_owned())).json()
    }

    #[get("/<series_id>")]
    fn select(db: DB, series_id: i32) -> JSON<ApiResult<serde_json::Value, String>> {
        let conn = db.conn();

        let series: Series = match series::dsl::series.filter(series::id.eq(series_id))
            .select(series::all_columns)
            .first(conn) {
            Ok(s) => s,
            Err(e) => return ApiResult::err_format(e).json(),
        };

        let info_uris: Vec<InfoUri> = match InfoUri::belonging_to(&series).load(conn) {
            Ok(i) => i,
            Err(e) => return ApiResult::err_format(e).json(),
        };

        let mut json = serde_json::to_value(series).unwrap();
        json.as_object_mut()
            .unwrap()
            .insert("info_uri".to_owned(),
                    serde_json::to_value(info_uris).unwrap());

        ApiResult::ok(json).json()
    }

    #[post("/new", data="<series_form>")]
    fn new(db: DB, series_form: JSON<SeriesForm>) -> JSON<ApiResult<serde_json::Value, String>> {
        let series_form = series_form.into_inner();
        let (new_series, info_uris) = series_form.into_new();

        if let Err(e) = new_series.validate() {
            return ApiResult::err_format(e).json();
        }

        let conn = db.conn();
        let new_series: Series = match insert(&new_series)
            .into(series::table)
            .returning(series::all_columns)
            .get_result(conn) {
            Ok(s) => s,
            Err(e) => return ApiResult::err_format(e).json(),
        };

        let new_info_uris: Vec<InfoUri> = match info_uris {
            Some(uris) => {
                let info_uris = uris.into_iter()
                    .map(|i| i.into_insertable(&new_series))
                    .collect::<Vec<NewInfoUri>>();
                match insert(&info_uris)
                    .into(info_uri::table)
                    .returning(info_uri::all_columns)
                    .get_results(conn) {
                    Ok(uris) => uris,
                    Err(e) => return ApiResult::err_format(e).json(),
                }
            }
            None => Vec::new(),
        };

        let mut result = serde_json::to_value(new_series).unwrap();
        result.as_object_mut()
            .unwrap()
            .insert("info_uri".to_owned(),
                    serde_json::to_value(new_info_uris).unwrap());

        ApiResult::ok(result).json()
    }

    #[put("/<series_id>", data="<series_form>")]
    fn update_series(db: DB,
                     series_id: i32,
                     series_form: JSON<SeriesForm>)
                     -> JSON<ApiResult<Series, String>> {
        let conn = db.conn();

        let (series_put, _) = series_form.into_inner().into_new();

        if let Err(e) = series_put.validate() {
            return ApiResult::err_format(e).json();
        }

        let series: Series = match update(series::dsl::series.filter(series::id.eq(series_id)))
            .set((series::title.eq(series_put.title),
                  series::episodes_current.eq(series_put.episodes_current),
                  series::episodes_total.eq(series_put.episodes_total),
                  series::start_date.eq(series_put.start_date),
                  series::end_date.eq(series_put.end_date)))
            .returning(series::all_columns)
            .get_result(conn) {
            Ok(s) => s,
            Err(e) => return ApiResult::err_format(e).json(),
        };

        ApiResult::ok(series).json()
    }

    pub fn routes() -> Vec<Route> {
        routes![all, new, select, update_series]
    }
}

pub fn routes() -> Vec<Route> {
    routes![index]
}
