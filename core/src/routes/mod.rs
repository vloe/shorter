use axum::{routing::get, Router};
use std::sync::Arc;

#[derive(Clone)]
pub struct Ctx {
    pub msg: Arc<String>,
}

pub fn mount(ctx: Ctx) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .with_state(ctx)
}
