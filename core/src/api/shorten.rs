use axum::{extract::Query, routing::get, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    domain: String,
}

pub fn mount() -> Router {
    Router::new().route(
        "/shorten",
        get(|Query(params): Query<Params>| async move {
            let domain = params.domain.trim();
            format!("domain: {}", domain)
        }),
    )
}
