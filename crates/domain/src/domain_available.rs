use bitvec::prelude::*;
use sh_crypto::hash::Hash;

pub const BITMAP_BYTE_SIZE: usize = 20_000_000; // 20 mb
pub const BITMAP_BIT_SIZE: usize = BITMAP_BYTE_SIZE * 8;

pub async fn domain_available(domain: &str, domains: &'static BitSlice<u8, Msb0>) -> bool {
    let index = Hash::domain_to_index(domain, BITMAP_BIT_SIZE);

    let is_registered = domains[index];
    let is_available = !is_registered;

    // Debug information
    println!(
        "Domain: {}, Index: {}, Is Registered: {}, Is Available: {}",
        domain, index, is_registered, is_available
    );

    // Print surrounding bits for context
    let start = index.saturating_sub(5);
    let end = (index + 6).min(domains.len());
    println!("Surrounding bits: {:?}", &domains[start..end]);

    // Print some statistics about the bitmap
    let set_bits = domains.count_ones();
    println!(
        "Total set bits in bitmap: {} out of {}",
        set_bits,
        domains.len()
    );

    is_available
}
