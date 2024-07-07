use crate::constants::tlds::TLDS;
use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use regex::Regex;
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
    domains: Vec<String>,
}

pub fn mount() -> Router {
    Router::new().route(
        "/shorten",
        get(|Query(params): Query<ShortenParams>| async move {
            let domain = params.domain.trim();

            // validate user input
            let domain_validation = match domain {
                d if d.is_empty() => Some("please provide a domain"),
                d if d.len() < 3 => Some("domain must be at least 3 characters"),
                d if d.len() > 64 => Some("domain must be at most 64 characters"),
                _ => None,
            };

            if let Some(err_msg) = domain_validation {
                return Err((StatusCode::BAD_REQUEST, err_msg));
            }

            // extract sld
            let mut sld = match Url::parse(domain) {
                Ok(url) => {
                    if let Some(d) = url.domain() {
                        extract_sld_from_domain(d)
                    } else {
                        return Err((StatusCode::BAD_REQUEST, "domain must be valid"));
                    }
                }
                Err(_) => {
                    let domain_regex = Regex::new(r"^[a-zA-Z0-9-]+(?:\.[a-zA-Z]{2,})?$").unwrap();
                    if domain_regex.is_match(domain) {
                        extract_sld_from_domain(domain)
                    } else {
                        return Err((StatusCode::BAD_REQUEST, "domain must be valid"));
                    }
                }
            };

            // the algorithm
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
                return Err((StatusCode::BAD_REQUEST, "could not find a shorter domain"));
            }

            let res = ShortenRes { domains };
            Ok(Json(res))
        }),
    )
}

fn extract_sld_from_domain(domain: &str) -> String {
    let parts: Vec<&str> = domain.split('.').collect();
    if parts.len() == 1 {
        return domain.to_string();
    }
    if parts.len() == 2 {
        return parts[0].to_string();
    }
    parts[parts.len() - 2].to_string()
}
