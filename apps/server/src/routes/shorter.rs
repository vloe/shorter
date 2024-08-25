use crate::error::AppError;
use crate::{
    constants::tlds::TLDS,
    models::domain::{get_sld_tld, Domain},
};
use axum::{extract::Query, Json};
use regex::Regex;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

const DEFAULT_TLD: &str = "com";
const MIN_SLD_LEN: usize = 2;
const MIN_DOMAIN_LEN: usize = 3;
const MAX_DOMAIN_LEN: usize = 253;

#[typeshare]
#[derive(Deserialize)]
pub(crate) struct ShorterParams {
    q: String,
}

impl ShorterParams {
    fn validate(&self) -> Result<(), AppError> {
        let q = self.q.trim().to_lowercase();

        if q.len() < MIN_DOMAIN_LEN {
            return Err(AppError::DomainTooShort(MIN_DOMAIN_LEN));
        }
        if q.len() > MAX_DOMAIN_LEN {
            return Err(AppError::DomainTooLong(MAX_DOMAIN_LEN));
        }

        Ok(())
    }
}

#[typeshare]
#[derive(Serialize)]
pub(crate) struct ShorterRes {
    domains: Vec<Domain>,
}

pub(crate) async fn mount(
    Query(params): Query<ShorterParams>,
) -> Result<Json<ShorterRes>, AppError> {
    params.validate()?;

    let domain = sanitize_domain(&params.q)?;

    let domains = vec![Domain::new(&domain)];

    let res = ShorterRes { domains };
    Ok(Json(res))
}

fn sanitize_domain(q: &str) -> Result<String, AppError> {
    let mut domain = q.trim().to_lowercase();

    // remove invalid characters
    let re = Regex::new(r"[^a-z0-9.-]").unwrap();
    domain = re.replace_all(&domain, "").to_string();
    domain = domain.trim_matches(|c| c == '-' || c == '.').to_string();

    // ensure domain structure is: sld dot tld
    let parts: Vec<&str> = domain.split('.').collect();
    let sld = parts[0];
    if sld.len() < MIN_SLD_LEN {
        return Err(AppError::DomainTooShort(MIN_SLD_LEN));
    }
    let tld = parts
        .iter()
        .skip(1)
        .find(|&part| TLDS.get(part).is_some())
        .unwrap_or(&DEFAULT_TLD);
    domain = format!("{}.{}", sld, tld);

    if domain.len() < MIN_DOMAIN_LEN {
        return Err(AppError::DomainTooShort(MIN_DOMAIN_LEN));
    }

    Ok(domain)
}
