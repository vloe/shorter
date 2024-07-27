use bitvec::prelude::*;
use sh_crypto::hash::Hash;

pub const BITMAP_SIZE: usize = 150000000;

pub async fn domain_available(domain: &str, domains: &'static BitSlice<u8, Msb0>) -> bool {
    let index = Hash::domain_to_index(domain, BITMAP_SIZE);

    let is_registered = domains[index];
    let is_available = !is_registered;

    is_available
}
