use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize)]
struct ShortenParams {
    domain: String,
}

#[typeshare]
#[derive(Serialize)]
struct ShortenRes {
    message: String,
}

pub fn mount() -> Router {
    Router::new().route(
        "/shorten",
        get(|Query(params): Query<ShortenParams>| async move {
            let domain = params.domain.trim();

            // validate
            if domain.is_empty() {
                return Err((StatusCode::BAD_REQUEST, "Domain is required"));
            }

            let res = Json(ShortenRes {
                message: format!("hey your domain is: {}", domain),
            });

            return Ok(res);
        }),
    )
}
