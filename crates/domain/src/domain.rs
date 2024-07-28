use crate::tlds::Tld;
use memmap2::Mmap;
use serde::Serialize;
use sh_crypto::hash::Hash;
use typeshare::typeshare;

pub const DOMAINS_BYTE_SIZE: usize = 20_000_000; // 20 mb
pub const DOMAINS_BIT_SIZE: usize = DOMAINS_BYTE_SIZE * 8;

#[typeshare]
#[derive(Serialize)]
pub struct Domain {
    pub name: String,
    pub tld: Tld,
    pub available: bool,
}

impl Domain {
    pub fn new(name: String, tld: Tld, domains: &Mmap) -> Self {
        let available = Self::check_availability(&name, domains);

        Domain {
            name,
            tld,
            available,
        }
    }

    fn check_availability(name: &str, domains: &Mmap) -> bool {
        let index = Hash::domain_to_index(name, DOMAINS_BIT_SIZE);
        let byte_index = index / 8;
        let bit_index = index % 8;

        domains[byte_index] & (1 << bit_index) == 0
    }
}
