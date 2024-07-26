use memmap2::MmapOptions;
use sh_crypto::hash::Hash;
use std::fs::File;

const DOMAINS_FILE: &str = "../assets/domains.bin";
pub const BITMAP_SIZE: usize = 50_000_000; // 50mb

pub async fn domain_available(domain: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let file = File::open(DOMAINS_FILE)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    let index = Hash::domain_to_index(domain, BITMAP_SIZE);
    let byte_index = index / 8;
    let bit_index = index % 8;

    let is_available = (mmap[byte_index] & (1 << bit_index)) != 0;

    Ok(is_available)
}
