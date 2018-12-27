use serde::Serialize;
use serde_json::json;

use crate::error::Result;

pub fn api_response<T: Serialize>(result: Result<T>) -> impl warp::Reply {
    let value = match result {
        Ok(data) => json!({
            "success": true,
            "data": data
        }),
        Err(err) => json!({
            "success": false,
            "error": err.description()
        }),
    };

    warp::reply::json(&value)
}
