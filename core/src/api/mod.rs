use axum::{routing::get, Router};
use hickory_resolver::TokioAsyncResolver;

mod shorten;

#[derive(Clone)]
pub struct Ctx {
    pub resolver: TokioAsyncResolver,
}

pub fn mount(ctx: Ctx) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/shorten", get(shorten::shorten))
        .with_state(ctx)
}
