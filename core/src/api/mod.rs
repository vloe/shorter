use axum::{routing::get, Router};

mod shorten;

pub fn mount() -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .merge(shorten::mount())
}
