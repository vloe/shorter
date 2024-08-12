use axum::Router;
use memmap2::Mmap;
use std::sync::Arc;

pub mod shorter;

#[derive(Clone)]
pub struct Ctx {
    pub domains: Arc<Mmap>,
}

pub fn mount(ctx: Ctx) -> Router {
    Router::new()
        .route("/", get(shorter::mount()))
        .with_state(ctx)
}
