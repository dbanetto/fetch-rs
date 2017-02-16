use rocket::Route;
use rocket_contrib::JSON;
use util::ApiResult;
use chrono::NaiveDate;

#[get("/")]
fn index() -> JSON<ApiResult<String, String>> {
    ApiResult::json(Ok("API available".to_owned()))
}

pub mod series {

    use std::error::Error;
    use db::DB;
    use util::ApiResult;
    use rocket::Route;
    use rocket::request::Form;
    use rocket_contrib::JSON;
    use models::*;
    use diesel::prelude::*;
    use schema::{info_uri, series};
    use diesel::result::QueryResult;
    use diesel::insert;
    use diesel::associations::BelongsTo;
    use serde_json;
    use diesel;

    #[get("/")]
    fn all(db: DB) -> JSON<ApiResult<Vec<Series>, String>> {
        let conn = db.conn();

        let result = series::dsl::series.load::<Series>(conn);

        ApiResult::json(result.map_err(|e| e.description().to_owned()))
    }

    #[get("/<series_id>")]
    fn select(db: DB, series_id: i32) -> JSON<ApiResult<serde_json::Value, String>> {
        let conn = db.conn();

        let series: Series = match series::dsl::series.filter(series::id.eq(series_id))
            .select(series::all_columns)
            .first(conn) {
            Ok(s) => s,
            Err(e) => return ApiResult::json(Err(e.description().to_owned())),
        };

        let info_uris: Vec<InfoUri> = match InfoUri::belonging_to(&series).load(conn) {
            Ok(i) => i,
            Err(e) => return ApiResult::json(Err(e.description().to_owned())),
        };

        let mut json = serde_json::to_value(series);
        json.as_object_mut()
            .unwrap()
            .insert("info_uri".to_owned(), serde_json::to_value(info_uris));

        ApiResult::json(Ok(json))
    }

    #[post("/new", data="<series_form>")]
    fn new(db: DB, series_form: JSON<SeriesForm>) -> JSON<ApiResult<serde_json::Value, String>> {
        let series_form = series_form.unwrap();
        let (new_series, info_uris) = series_form.into_new();

        if let Err(e) = new_series.validate() {
            return ApiResult::json(Err(e.description().to_owned()));
        }

        let conn = db.conn();
        let new_series: Series = match insert(&new_series)
            .into(series::table)
            .returning(series::all_columns)
            .get_result(conn) {
            Ok(s) => s,
            Err(e) => return ApiResult::json(Err(e.description().to_owned())),
        };

        let info_uris = info_uris.into_iter()
            .map(|i| i.to_insertable(&new_series))
            .collect::<Vec<NewInfoUri>>();

        let new_info_uris: Vec<InfoUri> = match insert(&info_uris)
            .into(info_uri::table)
            .returning(info_uri::all_columns)
            .get_results(conn) {
            Ok(uris) => uris,
            Err(e) => return ApiResult::json(Err(e.description().to_owned())),
        };

        let mut result = serde_json::to_value(new_series);
        result.as_object_mut()
            .unwrap()
            .insert("info_uri".to_owned(), serde_json::to_value(new_info_uris));

        ApiResult::json(Ok(result))
    }


    pub fn routes() -> Vec<Route> {
        routes![all, new, select]
    }
}

pub fn routes() -> Vec<Route> {
    routes![index]
}
