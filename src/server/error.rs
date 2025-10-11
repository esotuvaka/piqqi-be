pub enum ApiError {
    BadRequest,
    NotFound,
    TooManyRequests,
    Forbidden,
    Unauthorised,
    InternalServerError,
}

impl From<worker::Error> for ApiError {
    fn from(_: worker::Error) -> Self {
        ApiError::InternalServerError
    }
}
