use axum::{extract::Path, routing::get, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    domain: String,
}

pub fn mount() -> Router {
    Router::new().route(
        "/:domain",
        get(|Path(params): Path<Params>| async move {
            let domain = params.domain.trim();

            format!("Domain extracted: {}", domain)
        }),
    )
}
