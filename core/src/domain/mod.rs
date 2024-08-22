use crate::constants::tlds::{Tld, TLDS};
use memmap2::Mmap;
use serde::Serialize;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use typeshare::typeshare;

pub const DOMAINS_BYTE_SIZE: usize = 200_000_000; // 200 mb
pub const DOMAINS_BIT_SIZE: usize = DOMAINS_BYTE_SIZE * 8;
pub const NUM_HASH_FUNCTIONS: usize = 3;

#[typeshare]
#[derive(Serialize)]
pub struct Domain {
    pub name: String,
    pub tld: Tld,
    pub available: bool,
}

impl Domain {
    pub fn new(name: &str, domains: &Mmap) -> Option<Self> {
        let name = name.to_string();
        let tld = Self::get_tld(&name)?;
        let available = Self::is_available(&name, domains);

        let domain = Domain {
            name,
            tld,
            available,
        };
        Some(domain)
    }

    fn get_tld(name: &str) -> Option<Tld> {
        let parts: Vec<&str> = name.split('.').collect();

        if parts.len() != 2 {
            return None;
        }

        match TLDS.get(parts[1]) {
            Some(tld) => Some(tld.clone()),
            None => None,
        }
    }

    fn is_available(name: &str, domains: &Mmap) -> bool {
        let indices = domain_to_indices(name);
        let matched_count = indices
            .iter()
            .filter(|&&index| {
                let byte_index = index / 8;
                let bit_index = index % 8;
                domains[byte_index] & (1 << bit_index) != 0
            })
            .count();

        matched_count < NUM_HASH_FUNCTIONS
    }
}

pub fn domain_to_indices(domain: &str) -> [usize; NUM_HASH_FUNCTIONS] {
    let mut indices = [0; NUM_HASH_FUNCTIONS];
    for i in 0..NUM_HASH_FUNCTIONS {
        let mut hasher = DefaultHasher::new();
        domain.hash(&mut hasher);
        i.hash(&mut hasher);
        indices[i] = (hasher.finish() as usize) % DOMAINS_BIT_SIZE;
    }
    indices
}
