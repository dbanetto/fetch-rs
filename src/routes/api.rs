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

    #[get("/")]
    fn all(db: DB) -> JSON<ApiResult<Vec<Series>, String>> {
        use ::schema::series::dsl::*;
        let conn = db.conn();

        let result = series.load::<Series>(conn);

        ApiResult::json(result.map_err(|e| e.description().to_owned()))
    }

    #[post("/new", data="<series>")]
    fn new(_db: DB, series: Form<SeriesForm>) -> JSON<ApiResult<NewSeries, String>> {
        let new_series = NewSeries::from(series.into_inner());

        ApiResult::json(new_series.validate()
                        .map(move |_| new_series)
                        .map_err(|e| e.description().to_owned()))
    }

    pub fn routes() -> Vec<Route> {
        routes![all, new]
    }
}

pub fn routes() -> Vec<Route> {
    routes![index]
}
