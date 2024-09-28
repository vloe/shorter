use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("domain must be at least {0} characters")]
    DomainTooShort(usize),
    #[error("domain must be at most {0} characters")]
    DomainTooLong(usize),
    #[error("message must be at least {0} characters")]
    MsgTooShort(usize),
    #[error("message must be at most {0} characters")]
    MsgTooLong(usize),
    #[error("telegram error")]
    TelegramError(),
    #[error("env var not set: {0}")]
    EnvVarNotSet(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}
