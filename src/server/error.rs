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
