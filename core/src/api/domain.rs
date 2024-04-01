use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize)]
struct DomainArgs {
    domain: String,
}

#[typeshare]
#[derive(Serialize)]
struct DomainRes {
    domain_list: String,
}

pub(crate) fn mount() -> Router {
    Router::new().route(
        "/domain",
        post(|args: Json<DomainArgs>| async move {
            let domain = args.domain.trim();

            return Json(DomainRes {
                domain_list: domain.to_string(),
            });
        }),
    )
}
