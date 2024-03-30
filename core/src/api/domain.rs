use axum::{extract::Path, routing::get, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    domain: String,
}

pub fn mount() -> Router {
    Router::new().route(
        "/:domain",
        get(|Path(Params { domain }): Path<Params>| async move {
            format!("Domain extracted: {}", domain)
        }),
    )
}
