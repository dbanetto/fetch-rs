pub mod series;
pub mod info_uri;

use rocket::Route;
use rocket_contrib::Json;
use util::ApiResult;

#[get("/")]
fn index() -> Json<ApiResult<String, String>> {
    ApiResult::ok("API available".to_owned()).json()
}

pub fn routes() -> Vec<Route> {
    routes![index]
}
