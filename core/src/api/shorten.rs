use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize)]
struct ShortenParams {
    domain: String,
}

impl ShortenParams {
    fn validate(&self) -> Result<(), (StatusCode, String)> {
        const MIN_DOMAIN_LEN: usize = 4;
        const MAX_DOMAIN_LEN: usize = 255;

        let domain = self.domain.trim();
        if domain.len() < MIN_DOMAIN_LEN {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("domain must be at least {} characters", MIN_DOMAIN_LEN),
            ));
        }
        if domain.len() > MAX_DOMAIN_LEN {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("domain must be at most {} characters", MAX_DOMAIN_LEN),
            ));
        }

        Ok(())
    }
}

#[typeshare]
#[derive(Serialize)]
struct ShortenRes {
    domains: Vec<String>,
}

async fn shorten(
    Query(params): Query<ShortenParams>,
) -> Result<Json<ShortenRes>, (StatusCode, String)> {
    params.validate()?;

    let res = ShortenRes {
        domains: vec!["test.com".to_string()],
    };

    Ok(Json(res))
}

pub fn mount() -> Router {
    Router::new().route("/shorten", get(shorten))
}
