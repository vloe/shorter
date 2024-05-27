use axum::{routing::get, Router};

pub fn mount() -> Router {
    Router::new().route("/health", get(|| async { "ok" }))
}
