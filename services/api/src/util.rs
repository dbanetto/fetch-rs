use serde::Serialize;
use serde_json::{json, Value};
use warp::http::StatusCode;
use warp::reply;

use crate::error::{Error, Result};

pub fn api_response<T: Serialize>(result: Result<T>) -> impl warp::Reply {
    let (value, status) = match result {
        Ok(data) => (
            json!({
                "success": true,
                "data": data
            }),
            StatusCode::OK,
        ),
        Err(err) => handle_error(err),
    };

    reply::with_status(reply::json(&value), status)
}

fn handle_error(error: Error) -> (Value, StatusCode) {
    (
        json!({
            "success": false,
            "error": error.description()
        }),
        StatusCode::NOT_FOUND,
    )
}
