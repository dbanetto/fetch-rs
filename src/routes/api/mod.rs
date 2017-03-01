pub mod series;
pub mod info_uri;

use rocket::Route;
use rocket_contrib::JSON;
use util::ApiResult;

#[get("/")]
fn index() -> JSON<ApiResult<String, String>> {
    ApiResult::ok("API available".to_owned()).json()
}

pub fn routes() -> Vec<Route> {
    routes![index]
}
