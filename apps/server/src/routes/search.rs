use crate::{
    constants::tlds::{Tld, TLDS},
    error::AppError,
    Ctx,
};
use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

const MAX_DOMAIN: usize = 20;
const DEFAULT_TLD: &str = "com";

#[typeshare]
#[derive(Deserialize)]
pub struct SearchParams {
    q: String,
}

impl SearchParams {
    pub fn validate(&self) -> Result<(), AppError> {
        let trimmed = self.q.trim();
        if trimmed.is_empty() {
            return Err(AppError::IsEmpty("domain"));
        }
        if trimmed.len() > MAX_DOMAIN {
            return Err(AppError::TooLong("domain", MAX_DOMAIN));
        }
        Ok(())
    }
    pub fn sanitize(&self) -> Result<String, AppError> {
        let sanitized = {
            let mut trimmed = self.q.trim().to_lowercase();
            trimmed.retain(|c| c.is_ascii_alphanumeric() || c == '.');
            trimmed.trim_matches('.').to_string()
        };
        if sanitized.is_empty() {
            return Err(AppError::IsEmpty("domain"));
        }
        Ok(sanitized)
    }
}

#[typeshare]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub name: String,
    pub tld: Tld,
}

#[typeshare]
#[derive(Serialize)]
pub struct SearchRes {
    domains: Vec<Domain>,
}

pub async fn mount(
    State(ctx): State<Ctx>,
    Query(params): Query<SearchParams>,
) -> Result<Json<SearchRes>, AppError> {
    params.validate()?;

    let sanitized = params.sanitize()?;
    let (sld, tld, name) = get_domain_parts(&sanitized);
    let mut domains = vec![Domain { name, tld }];

    let res = SearchRes { domains };
    Ok(Json(res))
}

fn get_domain_parts(s: &str) -> (String, Tld, String) {
    let parts: Vec<&str> = s.split('.').collect();
    let sld = parts[0].to_string();
    let tld = parts
        .get(1)
        .and_then(|&p| TLDS.get(p).cloned())
        .unwrap_or_else(|| TLDS.get(DEFAULT_TLD).unwrap().clone());
    let name = format!("{}.{}", sld, &tld.name);
    (sld, tld, name)
}
