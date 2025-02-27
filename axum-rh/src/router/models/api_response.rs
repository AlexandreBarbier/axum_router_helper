pub use axum::{
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;

use super::ResponseBody;

#[derive(Default)]
pub struct ApiResponse<T> {
    pub status: StatusCode,
    pub body: ResponseBody<T>,
    pub error: bool,
    pub headers: Option<HeaderMap>,
}

impl<T> ApiResponse<T>
where
    T: Serialize + Default,
{
    fn new(status: StatusCode, data: Option<T>) -> Self {
        Self {
            body: ResponseBody::<T> {
                data,
                ..Default::default()
            },
            status,
            ..Default::default()
        }
    }

    pub fn ok(data: Option<T>) -> Self {
        Self::new(StatusCode::OK, data)
    }

    pub fn created(data: Option<T>) -> Self {
        Self::new(StatusCode::CREATED, data)
    }

    fn error(status: StatusCode, error: &str) -> Self {
        Self {
            body: ResponseBody {
                error: error.to_string(),
                ..Default::default()
            },
            status,
            error: true,
            ..Default::default()
        }
    }

    pub fn not_found(error: &str) -> Self {
        Self::error(StatusCode::NOT_FOUND, error)
    }

    pub fn bad_request(error: &str) -> Self {
        Self::error(StatusCode::BAD_REQUEST, error)
    }
    pub fn unprocessable(error: &str) -> Self {
        Self::error(StatusCode::UNPROCESSABLE_ENTITY, error)
    }

    pub fn service_error(error: &str) -> Self {
        Self::error(StatusCode::FAILED_DEPENDENCY, error)
    }

    pub fn unauthorized(error: &str) -> Self {
        Self::error(StatusCode::UNAUTHORIZED, error)
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize + Default,
{
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            self.headers.unwrap_or(HeaderMap::new()),
            self.body,
        )
            .into_response()
    }
}

impl<T> IntoResponse for ResponseBody<T>
where
    T: Serialize + Default,
{
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
