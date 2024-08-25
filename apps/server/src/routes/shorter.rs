use crate::constants::tlds::{Tld, TLDS};
use crate::error::AppError;
use axum::{extract::Query, Json};
use regex::Regex;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

const DEFAULT_TLD: &str = "com";
const MIN_SLD_LEN: usize = 2;
const MIN_DOMAIN_LEN: usize = 4;
const MAX_DOMAIN_LEN: usize = 253;

#[typeshare]
#[derive(Deserialize)]
pub(crate) struct ShorterParams {
    q: String,
}

impl ShorterParams {
    fn validate(&self) -> Result<Self, AppError> {
        let mut q = self.q.trim().to_lowercase();

        if q.len() < MIN_DOMAIN_LEN {
            return Err(AppError::DomainTooShort(MIN_DOMAIN_LEN));
        }
        if q.len() > MAX_DOMAIN_LEN {
            return Err(AppError::DomainTooLong(MAX_DOMAIN_LEN));
        }

        // remove invalid characters
        let re = Regex::new(r"[^a-z0-9.-]").unwrap();
        q = re.replace_all(&q, "").to_string();
        q = q.trim_matches(|c| c == '-' || c == '.').to_string();

        // ensure domain structure is: sld dot tld
        let parts: Vec<&str> = q.split('.').collect();
        let sld = parts[0];
        if sld.len() < MIN_SLD_LEN {
            return Err(AppError::DomainTooShort(MIN_SLD_LEN));
        }
        let tld = parts
            .iter()
            .skip(1)
            .find(|&part| TLDS.get(part).is_some())
            .unwrap_or(&DEFAULT_TLD);
        q = format!("{}.{}", sld, tld);

        if q.len() < MIN_DOMAIN_LEN {
            return Err(AppError::DomainTooShort(MIN_DOMAIN_LEN));
        }

        let params = Self { q };
        Ok(params)
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
    domains: Vec<Domain>,
}

pub(crate) async fn mount(
    Query(params): Query<ShorterParams>,
) -> Result<Json<ShorterRes>, AppError> {
    let params = params.validate()?;

    let domains = vec![
        Domain {
            name: params.q.to_string(),
            tld: TLDS.get("com").unwrap().clone(),
            available: false,
        },
        Domain {
            name: "example2.com".to_string(),
            tld: TLDS.get("com").unwrap().clone(),
            available: false,
        },
        Domain {
            name: "example3.com".to_string(),
            tld: TLDS.get("com").unwrap().clone(),
            available: false,
        },
    ];

    let res = ShorterRes { domains };
    Ok(Json(res))
}
