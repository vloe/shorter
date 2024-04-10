use crate::constants::tlds::{Tld, TLDS};
use axum::{http::StatusCode, routing::post, Json, Router};
use regex::Regex;
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
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Domain must be at least 3 characters",
                ));
            }

            if domain.len() > 64 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Domain must be at most 64 characters",
                ));
            }

            let domain_regex = Regex::new(r"^[a-z-]+(?:\.[a-z]+)?$").unwrap();
            if !domain_regex.is_match(&domain) {
                return Err((StatusCode::BAD_REQUEST, "Domain must be valid"));
            }

            // extract sld and tld

            Ok(Json(ShortenRes { message: "works!" }))
        }),
    )
}
