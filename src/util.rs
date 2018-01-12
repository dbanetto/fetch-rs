use serde::Serialize;
use serde_json;
use serde_json::Value;
use std::fmt::Display;
use iron::prelude::*;
use iron::status::Status;
use hyper::mime::{Attr, Mime, SubLevel, TopLevel, Value as MimeValue};
use std::error::Error;

#[derive(Serialize)]
pub struct ApiResult<T, E>
where
    T: Serialize,
    E: Serialize,
{
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<E>,
}

impl<T, E> ApiResult<T, E>
where
    T: Serialize,
    E: Serialize,
{

    pub fn new(data: Result<T, E>) -> Self {
        let (data, err) = match data {
            Ok(x) => (Some(x), None),
            Err(e) => (None, Some(e)),
        };

        ApiResult {
            success: data.is_some(),
            data: data,
            error: err,
        }
    }

    pub fn err(err: E) -> Self {
        ApiResult {
            data: None,
            error: Some(err),
            success: false,
        }
    }

    pub fn ok(data: T) -> Self {
        ApiResult {
            data: Some(data),
            error: None,
            success: true,
        }
    }

    pub fn json(self) -> Value {
        serde_json::to_value(self).unwrap()
    }

}


pub fn api_error<E: 'static + Error + Send>(error: E, status: Status) -> IronError {
    let description = format!("{}", error);
    let bytes = serde_json::to_vec(&ApiResult::<String, String>::err_format(description)).unwrap();
    IronError::new(error,
                   (status,
                    bytes,
                    Mime(
                        TopLevel::Application,
                        SubLevel::Json,
                        vec![(Attr::Charset, MimeValue::Utf8)],
                        ))
                  )
}

impl<T, E> Into<Response> for ApiResult<T, E>
where
    T: Serialize,
    E: Serialize,
{
    fn into(self) -> Response {
        let status = if self.success {
            Status::Ok
        } else {
            Status::InternalServerError
        };

        let bytes = serde_json::to_vec(&self).unwrap();

        Response::with((
            bytes,
            status,
            Mime(
                TopLevel::Application,
                SubLevel::Json,
                vec![(Attr::Charset, MimeValue::Utf8)],
            ),
        ))
    }
}

impl<T> ApiResult<T, String>
where
    T: Serialize,
{
    pub fn err_format<D: Display>(err: D) -> Self {
        ApiResult {
            data: None,
            error: Some(format!("{}", err)),
            success: false,
        }
    }
}

impl<T, E> From<Result<T, E>> for ApiResult<T, E>
where
    T: Serialize,
    E: Serialize,
{
    fn from(from: Result<T, E>) -> Self {
        ApiResult::new(from)
    }
}
