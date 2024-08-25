use crate::constants::tlds::{Tld, TLDS};
use memmap2::Mmap;
use serde::Serialize;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use typeshare::typeshare;

const DOMAINS_BYTE_SIZE: usize = 665_000_000; // 665 mb
const DOMAINS_BIT_SIZE: usize = DOMAINS_BYTE_SIZE * 8;
const NUM_HASH_FUNCTIONS: usize = 7;

#[typeshare]
#[derive(Serialize)]
pub(crate) struct Domain {
    name: String,
    tld: Tld,
    available: bool,
}

impl Domain {
    pub(crate) fn new(name: &str, domains: &Mmap) -> Self {
        let name = name.to_string();
        let tld = Self::get_tld(&name);
        let available = Self::is_available(&name, domains);

        Self {
            name,
            tld,
            available,
        }
    }

    fn get_tld(name: &str) -> Tld {
        let (_, tld) = get_sld_tld(name);
        TLDS.get(&tld).unwrap().clone()
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

pub(crate) fn get_sld_tld(domain: &str) -> (String, String) {
    let parts: Vec<&str> = domain.split('.').collect();
    (parts[0].to_string(), parts[1].to_string())
}

fn domain_to_indices(domain: &str) -> [usize; NUM_HASH_FUNCTIONS] {
    let mut indices = [0; NUM_HASH_FUNCTIONS];
    for i in 0..NUM_HASH_FUNCTIONS {
        let mut hasher = DefaultHasher::new();
        domain.hash(&mut hasher);
        i.hash(&mut hasher);
        indices[i] = (hasher.finish() as usize) % DOMAINS_BIT_SIZE;
    }
    indices
}
