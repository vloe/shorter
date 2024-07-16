use super::Ctx;
use crate::constants::tlds::{Tld, TLDS};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use futures::future::join_all;
use hickory_resolver::{error::ResolveError, proto::rr::RecordType, TokioAsyncResolver};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    status: Status,
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
    State(ctx): State<Ctx>,
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

    const MAX_VOWELS: usize = 15;
    let mut domain_futures = vec![];

    for perm in vowel_perms(sld, MAX_VOWELS) {
        for i in (1..perm.len()).rev() {
            let (new_sld, new_tld) = (&perm[..i], format!(".{}", &perm[i..]));
            let new_domain = format!("{}{}", new_sld, new_tld);

            if let Some(tld) = TLDS.get(&new_tld) {
                let resolver = ctx.resolver.clone();
                domain_futures.push(tokio::spawn(async move {
                    Domain {
                        name: new_domain.clone(),
                        tld: tld.clone(),
                        status: get_status(&new_domain, &resolver).await.unwrap(),
                    }
                }))
            }
        }
    }

    let domains: Vec<Domain> = join_all(domain_futures)
        .await
        .into_iter()
        .filter_map(|res| res.ok())
        .collect();

    if domains.is_empty() {
        return Err(ShortenErr::NoDomainFound);
    }

    Ok(Json(ShortenRes { domains }))
}

#[typeshare]
#[derive(Serialize)]
pub(crate) enum Status {
    Available,
    Unavailable,
}

async fn get_status(domain: &str, resolver: &TokioAsyncResolver) -> Result<Status, ResolveError> {
    // Check for NS records
    let ns_result = resolver.lookup(domain, RecordType::NS).await;

    // Check for SOA records if NS lookup fails
    if ns_result.is_err() {
        let soa_result = resolver.lookup(domain, RecordType::SOA).await;
        if soa_result.is_ok() {
            return Ok(Status::Unavailable);
        }
    } else {
        return Ok(Status::Unavailable);
    }

    // If both NS and SOA lookups fail, the domain is likely available
    Ok(Status::Available)
}

fn vowel_perms(word: String, max_perms: usize) -> Vec<String> {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let word_len = word.len();
    let mut res = Vec::with_capacity(word_len * (word_len + 1) / 2);
    res.push(word.clone());

    let mut v: Vec<char> = Vec::with_capacity(word_len);
    let mut v_pos: HashMap<usize, usize> = HashMap::with_capacity(word_len);

    for (i, ch) in word.chars().enumerate() {
        if VOWELS.contains(&ch) {
            v_pos.insert(v.len(), i);
            v.push(ch);
        }
    }

    let mut count = 0;
    for i in (0..v.len()).rev() {
        let mut wo_i = word.clone();
        wo_i.remove(v_pos[&i]);
        res.push(wo_i.clone());
        count += 1;

        for j in (i + 1..v.len()).rev() {
            let mut wo_ij = wo_i.clone();
            wo_ij.remove(v_pos[&j] - 1);
            res.push(wo_ij);
            count += 1;

            if count >= max_perms {
                return res;
            }
        }

        if count >= max_perms {
            return res;
        }
    }

    res
}
