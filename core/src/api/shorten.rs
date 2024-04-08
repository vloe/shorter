use crate::constants::tlds::{Scores, Tld, TLDS};
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
    domain_list: Vec<Tld>,
}

pub(crate) fn mount() -> Router {
    Router::new().route(
        "/shorten",
        post(|args: Json<ShortenArgs>| async move {
            let domain = args.domain.trim();

            Json(ShortenRes {
                domain_list: vec![Tld {
                    domain: "test.com",
                    category: "generic",
                    manager: "Able Inc.",
                    scores: Scores {
                        length: 0,
                        popularity: 0,
                        similarity: 0,
                        total: 0,
                    },
                }],
            })
        }),
    )
}
