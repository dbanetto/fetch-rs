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
    use models::{SeriesForm, NewSeries, Series};
    use diesel::prelude::*;
    use schema::series::dsl::*;
    use schema::series;
    use diesel::insert;
    use diesel;

    #[get("/")]
    fn all(db: DB) -> JSON<ApiResult<Vec<Series>, String>> {
        let conn = db.conn();

        let result = series.load::<Series>(conn);

        ApiResult::json(result.map_err(|e| e.description().to_owned()))
    }

    #[get("/<series_id>")]
    fn select(db: DB, series_id: i32) -> JSON<ApiResult<Series, String>> {
        let conn = db.conn();

        let result = series.filter(id.eq(series_id)).select(series::all_columns).first(conn);

        ApiResult::json(result.map_err(|e| e.description().to_owned()))
    }

    #[post("/new", data="<series_form>")]
    fn new(db: DB, series_form: JSON<SeriesForm>) -> JSON<ApiResult<Series, String>> {
        let new_series = NewSeries::from(series_form.unwrap());

        let validate = new_series.validate();

        if validate.is_err() {
            return ApiResult::json(Err(validate.unwrap_err().description().to_owned()));
        }

        let conn = db.conn();
        let series_id = insert(&new_series)
            .into(series::table)
            .returning(series::all_columns)
            .get_result(conn);

        ApiResult::json(series_id.map_err(|e| e.description().to_owned()))
    }


    pub fn routes() -> Vec<Route> {
        routes![all, new, select]
    }
}

pub fn routes() -> Vec<Route> {
    routes![index]
}
