use crate::constants::tlds::{Tld, TLDS};
use crate::error::AppError;
use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

const MIN_DOMAIN_LEN: usize = 2;
const MAX_DOMAIN_LEN: usize = 253;

#[typeshare]
#[derive(Deserialize)]
pub(crate) struct ShorterParams {
    q: String,
}

impl ShorterParams {
    fn validate(&self) -> Result<(), AppError> {
        let q = self.q.trim();
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
    params.validate()?;

    let domains = vec![
        Domain {
            name: "example1.com".to_string(),
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
