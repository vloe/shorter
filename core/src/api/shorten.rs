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
        const MAX_DOMAIN_LEN: usize = 20; // for now, algo is so ass

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

fn generate_perms(s: &str) -> Vec<String> {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut res = Vec::new();

    // Base case: if the string is empty or has no vowels, return it as is
    if s.is_empty() || !s.chars().any(|c| vowels.contains(&c)) {
        res.push(s.to_string());
        return res;
    }

    // Recursive case: generate permutations by removing vowels
    for (i, c) in s.char_indices() {
        if vowels.contains(&c) {
            let new_str = format!("{}{}", &s[..i], &s[i + 1..]);
            let sub_permutations = generate_perms(&new_str);
            res.extend(sub_permutations);
        }
    }

    // Add the original string as a permutation
    res.push(s.to_string());

    // Remove duplicates
    res.sort();
    res.dedup();
    res
}

async fn shorten(
    Query(params): Query<ShortenParams>,
) -> Result<Json<ShortenRes>, (StatusCode, String)> {
    params.validate()?;

    let domain = params.domain.trim();
    let sld = match domain.split_once('.') {
        Some((sld, _)) => sld.to_lowercase(),
        None => domain.to_lowercase(),
    };

    let perms = generate_perms(&sld);
    let mut domains: Vec<String> = Vec::new();
    for p in perms {
        let chars: Vec<char> = p.chars().collect();
        for i in (0..chars.len()).rev() {
            let suffix: String = chars[i..].iter().collect();
            if let Some(tld) = TLDS.get(&suffix) {
                let domain = format!("{}.{}", &p[..i], tld);
                domains.push(domain);
            }
        }
    }

    let res = ShortenRes { domains };

    Ok(Json(res))
}

pub fn mount() -> Router {
    Router::new().route("/shorten", get(shorten))
}
