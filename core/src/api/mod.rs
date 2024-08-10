use axum::Router;

pub mod shorten;

pub fn mount() -> Router {
    Router::new()
}
