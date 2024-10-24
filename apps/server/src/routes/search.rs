use crate::{
    constants::tlds::{Tld, TLDS},
    error::AppError,
};
use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use typeshare::typeshare;

pub const MAX_DOMAIN: usize = 20;
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

pub async fn mount(Query(params): Query<SearchParams>) -> Result<Json<SearchRes>, AppError> {
    params.validate()?;

    let sanitized = params.sanitize()?;
    let (sld, tld, name) = get_first_domain(&sanitized);
    let mut domains = vec![Domain { name, tld }];

    for perm in domain_perms(&sld) {
        let parts: Vec<&str> = perm.split('.').collect();
        let potential_tld = parts[1];
        if let Some(tld) = TLDS.get(potential_tld) {
            domains.push(Domain {
                name: perm,
                tld: tld.clone(),
            });
        }
    }

    let res = SearchRes { domains };
    Ok(Json(res))
}

fn get_first_domain(s: &str) -> (String, Tld, String) {
    let parts: Vec<&str> = s.split('.').collect();
    let sld = parts[0].to_string();
    let tld = parts
        .get(1)
        .and_then(|&part| TLDS.get(part))
        .unwrap_or_else(|| TLDS.get(DEFAULT_TLD).unwrap())
        .clone();
    let name = format!("{}.{}", sld, tld.name);
    (sld, tld, name)
}

fn vowel_perms(s: &str) -> Vec<String> {
    let mut res = vec![s.to_string()];

    let vowels_pos: Vec<usize> = s
        .chars()
        .enumerate()
        .filter(|&(_, char)| "aeiou".contains(char))
        .map(|(index, _)| index)
        .collect();
    let n = vowels_pos.len();

    for i in (0..n).rev() {
        let mut removed = s.to_string();
        removed.remove(vowels_pos[i]);
        res.push(removed.clone());
        for j in (i + 1..n).rev() {
            removed.remove(vowels_pos[j] - 1);
            res.push(removed.clone());
        }
    }

    res
}

fn dot_perms(s: &str) -> Vec<String> {
    let mut res = Vec::new();

    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();

    for i in (1..n - 1).rev() {
        let mut perm = String::new();
        for (j, &c) in chars.iter().enumerate() {
            if j == i {
                perm.push('.');
            }
            perm.push(c);
        }
        res.push(perm);
    }

    res
}

fn domain_perms(s: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut visited = HashSet::new();

    for v in vowel_perms(s) {
        for d in dot_perms(&v) {
            if !visited.contains(&d) {
                visited.insert(d.clone());
                res.push(d);
            }
        }
    }

    res
}
