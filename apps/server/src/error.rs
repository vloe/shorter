use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("internal error: {0}")]
    Internal(String),

    #[error("please provide a {0}")]
    IsEmpty(&'static str),

    #[error("{0} must be at most {1} characters")]
    TooLong(&'static str, usize),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::IsEmpty(_) => StatusCode::BAD_REQUEST,
            Self::TooLong(_, _) => StatusCode::BAD_REQUEST,
        };
        (status, self.to_string()).into_response()
    }
}
