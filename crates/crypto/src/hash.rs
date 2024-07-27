use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as StdHash, Hasher};

pub struct Hash;

impl Hash {
    // used to map a domain to an index in the `domains` bitmap
    pub fn domain_to_index(domain: &str, bitmap_size: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        domain.hash(&mut hasher);
        (hasher.finish() as usize) % bitmap_size
    }
}
