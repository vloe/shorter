use super::Ctx;
use crate::{constants::tlds::TLDS, domain::Domain};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use typeshare::typeshare;

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
    fn validate(&self) -> Result<(), ShorterErr> {
        let q = self.q.trim();
        if q.len() < MIN_DOMAIN_LEN {
            return Err(ShorterErr::DomainTooShort(MIN_DOMAIN_LEN));
        }
        if q.len() > MAX_DOMAIN_LEN {
            return Err(ShorterErr::DomainTooLong(MAX_DOMAIN_LEN));
        }
        Ok(())
    }
}

#[typeshare]
#[derive(Serialize)]
pub(crate) struct ShorterRes {
    domain: Domain,
    shorter_domains: Vec<Domain>,
}

pub(crate) async fn mount(
    Query(params): Query<ShorterParams>,
    State(ctx): State<Ctx>,
) -> Result<Json<ShorterRes>, ShorterErr> {
    params.validate()?;

    let domain = get_domain(&params.q);
    let shorter_domains = vec![];

    let res = ShorterRes {
        domain,
        shorter_domains,
    };
    Ok(Json(res))
}

fn get_domain(q: &str) -> Domain {
    Domain {
        name: "test.com".to_string(),
        tld: TLDS.get("com").unwrap().clone(),
        available: false,
    }
}
