use axum::Router;
use memmap2::Mmap;
use std::sync::Arc;

pub mod shorten;

#[derive(Clone)]
pub struct Ctx {
    pub domains: Arc<Mmap>,
}

pub fn mount(ctx: Ctx) -> Router {
    Router::new().with_state(ctx)
}
