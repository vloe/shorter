use crate::constants::tlds::{Tld, TLDS};
use axum::{http::StatusCode, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize)]
struct ShortenArgs {
    domain: String,
}

#[typeshare]
#[derive(Serialize)]
struct ShortenRes {
    message: &'static str,
}

pub(crate) fn mount() -> Router {
    Router::new().route(
        "/shorten",
        post(|args: Json<ShortenArgs>| async move {
            let domain = args.domain.trim().to_lowercase();

            // validate
            if domain.len() < 3 {
                return Err((StatusCode::BAD_REQUEST, "Domain is too short"));
            }

            Ok(Json(ShortenRes { message: "works!" }))
        }),
    )
}
