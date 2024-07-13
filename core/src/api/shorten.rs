use super::Ctx;
use crate::constants::tlds::{Tld, TLDS};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use typeshare::typeshare;
use url::Url;

#[typeshare]
#[derive(Deserialize)]
pub(crate) struct ShortenParams {
    domain: String,
}

#[typeshare]
#[derive(Serialize)]
pub(crate) struct Domain {
    name: String,
    tld: Tld,
}

#[typeshare]
#[derive(Serialize)]
pub(crate) struct ShortenRes {
    domains: Vec<Domain>,
}

#[derive(Error, Debug)]
pub(crate) enum ShortenErr {
    #[error("please provide a domain")]
    EmptyDomain,
    #[error("domain must be at least {0} characters")]
    DomainTooShort(usize),
    #[error("domain must be at most {0} characters")]
    DomainTooLong(usize),
    #[error("domain must be valid")]
    UnvalidDomain,
    #[error("could not find a shorter domain")]
    NoDomainFound,
}

impl IntoResponse for ShortenErr {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

impl ShortenParams {
    fn validate(&self) -> Result<(), ShortenErr> {
        const MIN_DOMAIN_LEN: usize = 3;
        const MAX_DOMAIN_LEN: usize = 64;

        let domain = self.domain.trim();
        if domain.is_empty() {
            return Err(ShortenErr::EmptyDomain);
        }
        if domain.len() < MIN_DOMAIN_LEN {
            return Err(ShortenErr::DomainTooShort(MIN_DOMAIN_LEN));
        }
        if domain.len() > MAX_DOMAIN_LEN {
            return Err(ShortenErr::DomainTooLong(MAX_DOMAIN_LEN));
        }

        Ok(())
    }
}

pub(crate) async fn shorten(
    Query(params): Query<ShortenParams>,
    State(_ctx): State<Ctx>,
) -> Result<Json<ShortenRes>, ShortenErr> {
    params.validate()?;

    let domain = params.domain.trim();

    let mut sld = match Url::parse(&domain) {
        Ok(url) => {
            if let Some(d) = url.domain() {
                let parts: Vec<&str> = d.split('.').collect();
                parts[parts.len() - 2].to_string()
            } else {
                return Err(ShortenErr::UnvalidDomain);
            }
        }
        Err(_) => {
            let domain_regex = Regex::new(r"^[a-zA-Z0-9-]+(?:\.[a-zA-Z]{2,})?$").unwrap();
            if domain_regex.is_match(&domain) {
                let parts: Vec<&str> = domain.split('.').collect();
                parts[0].to_string()
            } else {
                return Err(ShortenErr::UnvalidDomain);
            }
        }
    };

    let mut domains = Vec::new();
    let mut checked_sld = String::new();
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    for i in (0..sld.len()).rev() {
        if sld == checked_sld {
            break;
        }

        for j in (1..sld.len()).rev() {
            if j == sld.len() - 1 || j == 0 {
                continue;
            }

            let (new_sld, new_tld) = sld.split_at(j);
            let new_tld = format!(".{}", new_tld);

            if let Some(tld) = TLDS.get(&new_tld) {
                let name = format!("{}{}", new_sld, new_tld);
                domains.push(Domain {
                    name: name.clone(),
                    tld: tld.clone(),
                });
            }
        }

        checked_sld = sld.clone();
        if let Some(char) = sld.chars().nth(i) {
            if VOWELS.contains(&char) {
                sld.remove(i);
            }
        }
    }

    if domains.is_empty() {
        return Err(ShortenErr::NoDomainFound);
    }

    let res = ShortenRes { domains };
    Ok(Json(res))
}
