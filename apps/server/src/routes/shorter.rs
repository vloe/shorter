use crate::error::AppError;
use crate::{
    constants::tlds::TLDS,
    models::domain::{get_sld_tld, Domain},
};
use axum::{extract::Query, Json};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typeshare::typeshare;

const DEFAULT_TLD: &str = "com";
const MIN_SLD_LEN: usize = 2;
const MIN_DOMAIN_LEN: usize = 3;
const MAX_DOMAIN_LEN: usize = 253;
const MAX_DOMAINS: usize = 10;

#[typeshare]
#[derive(Deserialize)]
pub(crate) struct ShorterParams {
    q: String,
}

impl ShorterParams {
    fn validate(&self) -> Result<(), AppError> {
        let q = self.q.trim().to_lowercase();

        if q.len() < MIN_SLD_LEN {
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
    let mut domains = vec![Domain::new(&domain)];

    let (sld, _) = get_sld_tld(&domain);
    for perm in vowel_perms(&sld) {
        for i in (MIN_SLD_LEN..perm.len() - 1).rev() {
            let potential_domain = add_char_at(&perm, '.', i);
            let (_, potential_tld) = get_sld_tld(&potential_domain);
            if TLDS.get(&potential_tld).is_some() {
                domains.push(Domain::new(&potential_domain));
            }
            if domains.len() >= MAX_DOMAINS {
                break;
            }
        }
        if domains.len() >= MAX_DOMAINS {
            break;
        }
    }

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

fn vowel_perms(word: &str) -> Vec<String> {
    let mut res = vec![word.to_string()];
    let mut vowels: Vec<char> = vec![];
    let mut vowels_pos: HashMap<usize, usize> = HashMap::new();

    for (i, char) in word.chars().enumerate() {
        if ['a', 'e', 'i', 'o', 'u'].contains(&char) {
            vowels_pos.insert(vowels.len(), i);
            vowels.push(char);
        }
    }

    for i in (0..vowels.len()).rev() {
        let remove_i = remove_char_at(word, vowels_pos[&i]);
        if remove_i.len() >= MIN_DOMAIN_LEN {
            res.push(remove_i.clone());
        }
        if res.len() >= MAX_DOMAINS {
            break;
        }

        for j in (i + 1..vowels.len()).rev() {
            let remove_ij = remove_char_at(&remove_i, vowels_pos[&j] - 1);
            if remove_ij.len() >= MIN_DOMAIN_LEN {
                res.push(remove_ij);
            }
            if res.len() >= MAX_DOMAINS {
                break;
            }
        }
    }

    res
}

fn remove_char_at(s: &str, pos: usize) -> String {
    s.chars()
        .enumerate()
        .filter(|&(i, _)| i != pos)
        .map(|(_, char)| char)
        .collect()
}

fn add_char_at(s: &str, char: char, pos: usize) -> String {
    let mut res = String::with_capacity(s.len() + 1);
    for (i, c) in s.char_indices() {
        if i == pos {
            res.push(char);
        }
        res.push(c);
    }
    res
}
