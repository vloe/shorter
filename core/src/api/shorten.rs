use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use url::Url;

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

            // validate user input
            let err_msg = match domain {
                d if d.is_empty() => "domain is required",
                d if d.len() < 3 => "domain must be at least 3 characters",
                d if d.len() > 255 => "domain must be at most 255 characters",
                _ => "",
            };

            if !err_msg.is_empty() {
                return Err((StatusCode::BAD_REQUEST, err_msg));
            }

            // extract domain

            let res = Json(ShortenRes {
                message: format!("hey your domain is: {}", domain),
            });

            return Ok(res);
        }),
    )
}
