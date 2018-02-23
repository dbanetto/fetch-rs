use serde::Serialize;
use serde_json;
use iron::prelude::*;
use iron::status::Status;
use hyper::mime::{Attr, Mime, SubLevel, TopLevel, Value as MimeValue};
use std::error::Error;

#[derive(Serialize)]
struct ApiResult<T, E>
where
    T: Serialize,
    E: Serialize,
{
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")] pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")] pub error: Option<E>,
}

impl<T, E> ApiResult<T, E>
where
    T: Serialize,
    E: Serialize,
{
    fn err(err: E) -> Self {
        ApiResult {
            data: None,
            error: Some(err),
            success: false,
        }
    }

    fn ok(data: T) -> Self {
        ApiResult {
            data: Some(data),
            error: None,
            success: true,
        }
    }
}


pub fn api_response<T: Serialize, E: 'static + Error + Send>(
    resp: Result<T, E>,
    err_status: Status,
) -> IronResult<Response> {
    match resp {
        Ok(data) => Ok(api_success(data)),
        Err(err) => Err(api_error(err, err_status)),
    }
}

pub fn api_error<E: 'static + Error + Send>(error: E, status: Status) -> IronError {
    let description = format!("{}", error);
    let bytes = serde_json::to_vec(&ApiResult::<String, String>::err(
        format!("{}", description),
    )).unwrap();
    IronError::new(
        error,
        (
            status,
            bytes,
            Mime(
                TopLevel::Application,
                SubLevel::Json,
                vec![(Attr::Charset, MimeValue::Utf8)],
            ),
        ),
    )
}

pub fn api_success<T: Serialize>(data: T) -> Response {
    let bytes = serde_json::to_vec(&ApiResult::<T, String>::ok(data)).unwrap();

    Response::with((
        bytes,
        Status::Ok,
        Mime(
            TopLevel::Application,
            SubLevel::Json,
            vec![(Attr::Charset, MimeValue::Utf8)],
        ),
    ))
}
