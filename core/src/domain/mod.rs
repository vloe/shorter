use crate::constants::tlds::{Tld, TLDS};
use serde::Serialize;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use typeshare::typeshare;

pub const DOMAINS_BYTE_SIZE: usize = 50_000_000; // 50 mb
pub const DOMAINS_BIT_SIZE: usize = DOMAINS_BYTE_SIZE * 8;

#[typeshare]
#[derive(Serialize)]
pub struct Domain {
    pub name: String,
    pub tld: Tld,
    pub available: bool,
}

impl Domain {
    pub fn new(name: String) -> Option<Self> {
        let tld = get_tld(&name)?;
        let available = Self::is_available(&name);

        let domain = Domain {
            name,
            tld,
            available,
        };
        Some(domain)
    }

    pub fn is_available(name: &str) -> bool {
        false
    }
}

pub fn get_tld(name: &str) -> Option<Tld> {
    let parts: Vec<&str> = name.split('.').collect();

    if parts.len() != 2 {
        return None;
    }

    match TLDS.get(parts[1]) {
        Some(tld) => Some(tld.clone()),
        None => None,
    }
}

pub fn domain_to_index(domain: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    domain.hash(&mut hasher);
    (hasher.finish() as usize) % DOMAINS_BIT_SIZE
}
