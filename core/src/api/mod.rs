use axum::{routing::get, Router};

mod shorten;

pub fn mount() -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .merge(shorten::mount())
}
