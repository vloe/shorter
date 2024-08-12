use super::Ctx;
use crate::constants::tlds::{Tld, TLDS};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use memmap2::Mmap;
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use thiserror::Error;
use typeshare::typeshare;

pub const DOMAINS_BYTE_SIZE: usize = 20_000_000; // 20 mb
pub const DOMAINS_BIT_SIZE: usize = DOMAINS_BYTE_SIZE * 8;
const MIN_DOMAIN_LEN: usize = 2;
const MAX_DOMAIN_LEN: usize = 255;

#[derive(Error, Debug)]
pub(crate) enum ShorterErr {
    #[error("domain must be at least {0} characters")]
    DomainTooShort(usize),
    #[error("domain must be at most {0} characters")]
    DomainTooLong(usize),
    #[error("could not find a shorter domain")]
    NoShorterDomainFound,
}

impl IntoResponse for ShorterErr {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

#[typeshare]
#[derive(Deserialize)]
pub(crate) struct ShorterParams {
    q: String,
}

impl ShorterParams {
    fn validate(&self) -> Result<String, ShorterErr> {
        let domain = self.q.trim().to_lowercase();
        if domain.len() < MIN_DOMAIN_LEN {
            return Err(ShorterErr::DomainTooShort(MIN_DOMAIN_LEN));
        }
        if domain.len() > MAX_DOMAIN_LEN {
            return Err(ShorterErr::DomainTooLong(MAX_DOMAIN_LEN));
        }
        Ok(domain)
    }
}

#[typeshare]
#[derive(Serialize)]
pub(crate) struct Domain {
    name: String,
    tld: Tld,
    available: bool,
}

#[typeshare]
#[derive(Serialize)]
pub(crate) struct ShorterRes {
    domain: Domain,
    shorter_domains: Vec<Domain>,
}

pub(crate) fn mount(
    Query(params): Query<ShorterParams>,
    State(ctx): State<Ctx>,
) -> Result<(), ShorterErr> {
    let domain = params.validate()?;

    Ok(())
}

pub fn domain_to_index(domain: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    domain.hash(&mut hasher);
    (hasher.finish() as usize) % DOMAINS_BIT_SIZE
}
