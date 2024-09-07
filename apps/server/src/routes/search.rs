use std::env::remove_var;

use crate::constants::tld_info::{TldInfo, TLD_INFO};
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use typeshare::typeshare;

const MIN_DOMAIN_LEN: usize = 3;
const MAX_DOMAIN_LEN: usize = 255;
const DEFAULT_TLD: &str = "com";

#[derive(Error, Debug)]
pub enum SearchErr {
    #[error("domain must be at least {0} characters")]
    DomainTooShort(usize),
    #[error("domain must be at most {0} characters")]
    DomainTooLong(usize),
}

impl IntoResponse for SearchErr {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

#[typeshare]
#[derive(Deserialize)]
pub struct SearchParams {
    q: String,
}

impl SearchParams {
    pub fn validate(&self) -> Result<(), SearchErr> {
        let q = self.q.trim();
        if q.len() < MIN_DOMAIN_LEN {
            return Err(SearchErr::DomainTooShort(MIN_DOMAIN_LEN));
        }
        if q.len() > MAX_DOMAIN_LEN {
            return Err(SearchErr::DomainTooLong(MAX_DOMAIN_LEN));
        }
        Ok(())
    }
}

#[typeshare]
#[derive(Serialize)]
pub enum Status {
    Available,
    Unavailable,
    NotSureYet,
}

#[typeshare]
#[derive(Serialize)]
pub struct Domain {
    name: String,
    sld: String,
    tld: String,
    tld_info: TldInfo,
    status: Status,
}

impl Domain {
    pub fn new(domain: &str) -> Self {
        let name = domain.to_string();
        let (sld, tld) = get_sld_tld(&domain);
        let tld_info = Self::get_tld_info(&tld);
        let status = Self::get_status(&name);

        Domain {
            name,
            sld,
            tld,
            tld_info,
            status,
        }
    }

    fn get_tld_info(tld: &str) -> TldInfo {
        TLD_INFO.get(tld).unwrap().clone()
    }

    fn get_status(domain: &str) -> Status {
        Status::NotSureYet
    }
}

#[typeshare]
#[derive(Serialize)]
pub struct SearchRes {
    domains: Vec<Domain>,
}

pub async fn mount(Query(params): Query<SearchParams>) -> Result<Json<SearchRes>, SearchErr> {
    params.validate()?;

    let domain = sanitize_domain(&params.q)?;
    let mut domains = vec![Domain::new(&domain)];

    let (sld, _) = get_sld_tld(&domain);
    for perm in vowel_removal_perms(&sld) {
        for i in 1..perm.len() - 1 {
            let (potential_sld, potential_tld) = perm.split_at(i);
            let potential_domain = format!("{}.{}", potential_sld, potential_tld);
            if TLD_INFO.get(&potential_tld).is_some() {
                domains.push(Domain::new(&potential_domain));
            }
        }
    }

    let res = SearchRes { domains };
    Ok(Json(res))
}

fn sanitize_domain(q: &str) -> Result<String, SearchErr> {
    let mut s = q.trim().to_lowercase();
    s.retain(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.');
    s.trim_matches(|c| c == '.' || c == '-').to_string();

    // ensure the correct domain structure: sld dot tld
    let parts: Vec<&str> = s.split('.').collect();
    let sld = parts[0];
    let tld = parts
        .iter()
        .skip(1)
        .find(|&part| TLD_INFO.get(part).is_some())
        .unwrap_or(&DEFAULT_TLD);
    let domain = format!("{}.{}", sld, tld);

    if domain.len() < MIN_DOMAIN_LEN {
        return Err(SearchErr::DomainTooShort(MIN_DOMAIN_LEN));
    }

    Ok(domain)
}

fn get_sld_tld(domain: &str) -> (String, String) {
    let parts: Vec<&str> = domain.split('.').collect();
    (parts[0].to_string(), parts[1].to_string())
}

fn vowel_removal_perms(s: &str) -> Vec<String> {
    let mut res = vec![s.to_string()];
    let vowels_pos: Vec<usize> = s
        .chars()
        .enumerate()
        .filter(|&(_, c)| "aeiou".contains(c))
        .map(|(i, _)| i)
        .collect();

    for i in (0..vowels_pos.len()).rev() {
        let mut remove = s.to_string();
        remove.remove(vowels_pos[i]);
        res.push(remove.clone());
        for j in (i + 1..vowels_pos.len()).rev() {
            remove.remove(vowels_pos[j] - 1);
            res.push(remove.clone());
        }
    }

    res
}
