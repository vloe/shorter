use axum::{routing::post, Json, Router};
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
    domain_list: String,
}

pub(crate) fn mount() -> Router {
    Router::new().route(
        "/shorten",
        post(|args: Json<ShortenArgs>| async move {
            let domain = args.domain.trim();

            return Json(ShortenRes {
                domain_list: domain.to_string(),
            });
        }),
    )
}
