use super::Ctx;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use futures::future::join_all;
use hickory_resolver::{proto::rr::RecordType, TokioAsyncResolver};
use serde::{Deserialize, Serialize};
use sh_domain::{
    domain_available::{self, domain_available},
    tlds::{Tld, TLDS},
};
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
    let sld = extract_sld(&domain)?;

    const MAX_VOWELS: usize = 6;
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
                        status: get_status(&new_domain, &resolver).await,
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

fn extract_sld(domain: &str) -> Result<String, ShortenErr> {
    /* function returns the sld (second level domain) of a domain.
    works on all: example, example.com, http://example.com etc... */
    if !domain.contains('.') {
        return Ok(domain.to_string());
    }

    let url = Url::parse(domain)
        .or_else(|_| Url::parse(&format!("http://{}", domain)))
        .map_err(|_| ShortenErr::UnvalidDomain)?;

    if let Some(d) = url.domain() {
        let parts: Vec<&str> = d.split('.').collect();
        for i in 1..parts.len() {
            let potential_tld = format!(".{}", parts[i]);
            if TLDS.get(&potential_tld).is_some() {
                return Ok(parts[i - 1].to_string());
            }
        }
    }

    Err(ShortenErr::UnvalidDomain)
}

#[typeshare]
#[derive(Serialize)]
pub(crate) enum Status {
    Available,
    Unavailable,
}

async fn get_status(domain: &str, resolver: &TokioAsyncResolver) -> Status {
    //let domain_available = domain_available(domain, "../../crates/domain/src/assets/domains.bin")
    //    .await
    //    .unwrap();

    //if domain_available {
    //    return Status::Available;
    //}

    let ns_result = resolver.lookup(domain, RecordType::NS).await;

    if ns_result.is_err() {
        let soa_result = resolver.lookup(domain, RecordType::SOA).await;
        if soa_result.is_ok() {
            return Status::Unavailable;
        }
    } else {
        return Status::Unavailable;
    }

    Status::Available
}

fn vowel_perms(word: String, max_vowels: usize) -> Vec<String> {
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

    let v_len = v.len();
    for i in (0..v_len).rev() {
        let mut wo_i = word.clone();
        wo_i.remove(v_pos[&i]);
        res.push(wo_i.clone());

        for j in (i + 1..v_len).rev() {
            let mut wo_ij = wo_i.clone();
            wo_ij.remove(v_pos[&j] - 1);
            res.push(wo_ij);
        }

        if v_len - i >= max_vowels {
            break;
        }
    }

    res
}
