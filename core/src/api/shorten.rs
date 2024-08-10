use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub const DOMAINS_BYTE_SIZE: usize = 20_000_000; // 20 mb
pub const DOMAINS_BIT_SIZE: usize = DOMAINS_BYTE_SIZE * 8;

pub fn domain_to_index(domain: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    domain.hash(&mut hasher);
    (hasher.finish() as usize) % DOMAINS_BIT_SIZE
}
