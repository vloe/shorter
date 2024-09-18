use crate::{constants::tld_info::TLD_INFO, error::AppError, models::domain::Domain, Ctx};
use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

const MIN_DOMAIN_LEN: usize = 3;
const MAX_DOMAIN_LEN: usize = 30;
const DEFAULT_TLD: &str = "com";

#[typeshare]
#[derive(Deserialize)]
pub struct SearchParams {
    q: String,
}

impl SearchParams {
    pub fn validate(&mut self) -> Result<&mut Self, AppError> {
        self.q = self.q.trim().to_string();

        if self.q.len() < MIN_DOMAIN_LEN {
            return Err(AppError::DomainTooShort(MIN_DOMAIN_LEN));
        }
        if self.q.len() > MAX_DOMAIN_LEN {
            return Err(AppError::DomainTooLong(MAX_DOMAIN_LEN));
        }

        Ok(self)
    }

    pub fn sanitize(&mut self) -> Result<&mut Self, AppError> {
        // remove all chars that are not supposed to be in a domain
        self.q = {
            let mut s = self.q.to_lowercase();
            s.retain(|c| c.is_ascii_alphanumeric() || c == '-' || c == '.');
            s.trim_matches(|c| c == '.' || c == '-').to_string()
        };

        // ensure correct domain structure: sld dot tld
        let parts: Vec<&str> = self.q.split('.').collect();
        let sld = parts[0];
        let tld = parts
            .iter()
            .skip(1)
            .find(|&part| TLD_INFO.get(part).is_some())
            .unwrap_or(&DEFAULT_TLD);
        self.q = format!("{}.{}", sld, tld);

        if self.q.len() < MIN_DOMAIN_LEN {
            return Err(AppError::DomainTooShort(MIN_DOMAIN_LEN));
        }

        Ok(self)
    }
}

#[typeshare]
#[derive(Serialize)]
pub struct SearchRes {
    domains: Vec<Domain>,
}

pub async fn mount(
    Query(mut params): Query<SearchParams>,
    State(ctx): State<Ctx>,
) -> Result<Json<SearchRes>, AppError> {
    params.validate()?.sanitize()?;

    let domain = Domain::new(&params.q, &ctx).await;
    let mut domains = vec![domain.clone()];

    for perm in vowel_removal_perms(&domain.sld) {
        for i in 1..perm.len() - 1 {
            let (sld, tld) = perm.split_at(i);
            if TLD_INFO.get(&tld).is_some() {
                let name = format!("{}.{}", sld, tld);
                domains.push(Domain::new(&name, &ctx).await);
            }
        }
    }

    let res = SearchRes { domains };
    Ok(Json(res))
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
