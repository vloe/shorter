use memmap2::MmapOptions;
use sh_crypto::hash::Hash;
use std::fs::File;

pub const BITMAP_SIZE: usize = 20_000_000; // 20mb

pub async fn domain_available(
    domain: &str,
    path_to_domains: &str,
) -> Result<bool, Box<dyn std::error::Error>> {
    let file = File::open(path_to_domains)?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    let index = Hash::domain_to_index(domain, BITMAP_SIZE);
    let byte_index = index / 8;
    let bit_index = index % 8;

    let is_registered = (mmap[byte_index] & (1 << bit_index)) != 0;
    let is_available = !is_registered;

    Ok(is_available)
}
