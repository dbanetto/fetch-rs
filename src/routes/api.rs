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
    use diesel;

    #[get("/")]
    fn all(db: DB) -> JSON<ApiResult<Vec<Series>, String>> {
        let conn = db.conn();

        let result = series::dsl::series.load::<Series>(conn);

        ApiResult::json(result.map_err(|e| e.description().to_owned()))
    }

    #[get("/<series_id>")]
    fn select(db: DB, series_id: i32) -> JSON<ApiResult<Series, String>> {
        let conn = db.conn();

        let result = series::dsl::series.filter(series::id.eq(series_id)).select(series::all_columns).first(conn);

        ApiResult::json(result.map_err(|e| e.description().to_owned()))
    }

    #[post("/new", data="<series_form>")]
    fn new(db: DB,
           series_form: JSON<SeriesForm>)
           -> JSON<ApiResult<(Series, Vec<InfoUri>), String>> {
        let series_form = series_form.unwrap();
        let (new_series, info_uris) = series_form.into_new();

        let validate = new_series.validate();

        if validate.is_err() {
            return ApiResult::json(Err(validate.unwrap_err().description().to_owned()));
        }

        let conn = db.conn();
        let new_series: QueryResult<Series> = insert(&new_series)
            .into(series::table)
            .returning(series::all_columns)
            .get_result(conn);

        if new_series.is_err() {
            return ApiResult::json(Err(new_series.unwrap_err().description().to_owned()));
        }

        let new_series: Series = new_series.unwrap();

        let new_info_uris = {
            insert(&info_uris.into_iter().map(|i| i.to_insertable(&new_series)).collect::<Vec<NewInfoUri>>())
                .into(info_uri::table)
                .returning(info_uri::all_columns)
                .get_results(conn)
        };

        let result = new_info_uris.map(move |uris| (new_series, uris))
                                  .map_err(|e| e.description().to_owned()); 

        ApiResult::json(result)
    }


    pub fn routes() -> Vec<Route> {
        routes![all, new, select]
    }
}

pub fn routes() -> Vec<Route> {
    routes![index]
}
