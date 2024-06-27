use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
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
    message: String,
}

fn extract_sld(domain: &str) -> Result<String, &'static str> {
    let prefixes = ["https://", ""];
    let suffixes = [".com", "com", ""];

    for prefix in &prefixes {
        for suffix in &suffixes {
            let url_str = format!("{}{}{}", prefix, domain, suffix);
            if let Ok(url) = Url::parse(&url_str) {
                if let Some(host) = url.host_str() {
                    let parts: Vec<&str> = host.split('.').collect();
                    if parts.len() >= 2 {
                        let sld = parts[parts.len() - 2].to_string();
                        return Ok(sld);
                    }
                }
            }
        }
    }

    Err("domain must be valid")
}

pub fn mount() -> Router {
    Router::new().route(
        "/shorten",
        get(|Query(params): Query<ShortenParams>| async move {
            let domain = params.domain.trim();

            // validate user input
            let err_msg = match domain {
                d if d.is_empty() => "domain is required",
                d if d.len() < 3 => "domain must be at least 3 characters",
                d if d.len() > 255 => "domain must be at most 255 characters",
                _ => "",
            };

            if !err_msg.is_empty() {
                return Err((StatusCode::BAD_REQUEST, err_msg));
            }

            // extract sld
            let sld = match extract_sld(domain) {
                Ok(sld) => sld,
                Err(err_msg) => return Err((StatusCode::BAD_REQUEST, err_msg)),
            };

            let res = Json(ShortenRes {
                message: format!("sld: {}", sld),
            });

            return Ok(res);
        }),
    )
}
