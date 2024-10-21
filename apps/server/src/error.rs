use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("please provide a {0}")]
    IsEmpty(&'static str),

    #[error("{0} must be at least {1} characters")]
    TooShort(&'static str, usize),

    #[error("{0} must be at most {1} characters")]
    TooLong(&'static str, usize),

    #[error("{0} must be set")]
    EnvNotSet(&'static str),

    #[error("something went wrong, please try again later")]
    FuckUp(),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}
