use crate::constants::tlds::TLDS;
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
        const MAX_DOMAIN_LEN: usize = 64;

        let domain = self.domain.trim();
        if domain.is_empty() {
            return Err((StatusCode::BAD_REQUEST, "domain is required".to_string()));
        }
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
        if domain.contains(char::is_whitespace) {
            return Err((StatusCode::BAD_REQUEST, "domain must be valid".to_string()));
        }
        if !domain.contains(char::is_alphanumeric) {
            return Err((StatusCode::BAD_REQUEST, "domain must be valid".to_string()));
        }
        if domain.matches('.').count() > 1 {
            return Err((StatusCode::BAD_REQUEST, "domain must be valid".to_string()));
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

    let domain = params.domain.trim();
    println!("domain: {}", domain);

    // extract sld
    let mut sld = match domain.split_once('.') {
        Some((sld, _)) => sld.to_lowercase(),
        None => domain.to_lowercase(),
    };

    // the algorithm :o
    let mut domains = Vec::new();
    let mut checked_sld = String::new();
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    for i in (0..sld.len()).rev() {
        if sld == checked_sld {
            continue;
        }
        for j in (1..sld.len()).rev() {
            if j == sld.len() - 1 || j == 0 {
                continue;
            }
            let (new_sld, new_tld) = sld.split_at(j);
            if TLDS.get(new_tld).is_some() {
                domains.push(format!("{}.{}", new_sld, new_tld));
            }
        }
        checked_sld = sld.clone();
        if VOWELS.contains(&sld.chars().nth(i).unwrap()) {
            sld.remove(i);
        }
    }

    if domains.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "could not find a shorter domain".to_string(),
        ));
    }

    let res = ShortenRes { domains };

    Ok(Json(res))
}

pub fn mount() -> Router {
    Router::new().route("/shorten", get(shorten))
}
