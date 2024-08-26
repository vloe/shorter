use crate::constants::tld_info::{TldInfo, TLD_INFO};
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
    sld: String,
    tld: String,
    tld_info: TldInfo,
    available: bool,
}

impl Domain {
    pub(crate) fn new(name: &str, domains: &Mmap) -> Option<Self> {
        let name = name.to_string();
        let (sld, tld) = extract_sld_tld(&name);
        let tld_info = Self::get_tld_info(&tld)?;
        let available = Self::is_available(&name, domains);

        let domain = Self {
            name,
            sld,
            tld,
            tld_info,
            available,
        };
        Some(domain)
    }

    fn get_tld_info(tld: &str) -> Option<TldInfo> {
        TLD_INFO.get(&tld).cloned()
    }

    fn is_available(name: &str, domains: &Mmap) -> bool {
        let indices = Self::domain_to_indices(name);
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
}

pub(crate) fn extract_sld_tld(domain: &str) -> (String, String) {
    let parts: Vec<&str> = domain.split('.').collect();
    (parts[0].to_string(), parts[1].to_string())
}
