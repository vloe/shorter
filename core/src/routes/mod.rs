use axum::{routing::get, Router};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct Ctx {
    pub db: Arc<PgPool>,
}

pub fn mount(ctx: Ctx) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .with_state(ctx)
}
