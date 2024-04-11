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
            let err_msg = match domain {
                l if l.is_empty() => "Domain cannot be empty",
                l if l.len() < 3 => "Domain must be at least 3 characters",
                l if l.len() > 64 => "Domain must be at most 64 characters",
                r if r.matches('.').count() > 1 => "Domain cannot contain more than one dot",
                r if !Regex::new(r"^[a-z\-\.]+$").unwrap().is_match(&r) => {
                    "Domain cannot contain special characters or numbers"
                }
                r if !Regex::new(r"^[a-z-]+(?:\.[a-z]+)*$").unwrap().is_match(&r) => {
                    "Domain must use 'example' or 'example.com' format"
                }
                _ => "",
            };

            if !err_msg.is_empty() {
                return Err((StatusCode::BAD_REQUEST, err_msg));
            }

            Ok(Json(ShortenRes { message: "works!" }))
        }),
    )
}
