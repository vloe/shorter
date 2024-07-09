use axum::{routing::get, Router};

mod shorten;

#[derive(Clone)]
pub struct Ctx {}

pub fn mount(ctx: Ctx) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/shorten", get(shorten::shorten))
        .with_state(ctx)
}
