use serde::Serialize;
use rocket_contrib::JSON;
use std::fmt::Display;

#[derive(Serialize)]
pub struct ApiResult<T, E>
    where T: Serialize,
          E: Serialize
{
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<E>,
}

impl<T, E> ApiResult<T, E>
    where T: Serialize,
          E: Serialize
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

    pub fn json(self) -> JSON<Self> {
        JSON(self)
    }
}

impl<T> ApiResult<T, String>
    where T: Serialize
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
    where T: Serialize,
          E: Serialize
{
    fn from(from: Result<T, E>) -> Self {
        ApiResult::new(from)
    }
}
