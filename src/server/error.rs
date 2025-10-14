use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiError {
    BadRequest,
    NotFound,
    TooManyRequests,
    Forbidden,
    Unauthorised,
    InternalServerError(String),
}

impl From<worker::Error> for ApiError {
    fn from(e: worker::Error) -> Self {
        ApiError::InternalServerError(e.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiErrorResponse {
    error: ApiError,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        Json(ApiErrorResponse { error: self }).into_response()
    }
}
