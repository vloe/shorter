use axum::{routing::get, Router};

mod domain;

pub fn mount() -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(domain::mount())
}
