use crate::constants::tlds::Tld;
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

pub fn domain_to_index(domain: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    domain.hash(&mut hasher);
    (hasher.finish() as usize) % DOMAINS_BIT_SIZE
}
