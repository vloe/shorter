use dotenv::dotenv;
use flate2::read::GzDecoder;
use memmap2::MmapOptions;
use serde_json::{json, Value};
use sh_crypto::hash::Hash;
use sh_domain::domain_available::BITMAP_SIZE;
use std::{env, error::Error, fs::OpenOptions, io::prelude::*, path::Path};
use tokio::{
    fs::{self, File},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    const CZDS_API_URL: &str = "https://czds-api.icann.org/czds/downloads/links";
    const CZDS_API_AUTH_URL: &str = "https://account-api.icann.org/api/authenticate";
    const ZONE_DIR: &str = "../crates/domain/src/tmp/";
    const DOMAINS_FILE: &str = "../crates/domain/src/assets/domains.bin";

    let client = reqwest::Client::builder()
        .user_agent("curl/7.79.1")
        .build()?;

    let access_token = get_access_token(&client, CZDS_API_AUTH_URL).await?;
    let zone_urls = get_zone_urls(&client, CZDS_API_URL, &access_token).await?;

    for (i, zone_url) in zone_urls.iter().rev().enumerate() {
        let (file_name, file_path) = parse_zone_url(zone_url, ZONE_DIR).await?;

        if let Err(e) = download_zone(&client, zone_url, &access_token, &file_path).await {
            println!("failed to download zone: {}, err: {}", zone_url, e);
            continue;
        };
        if let Err(e) = bitmap_zone(&file_path, DOMAINS_FILE, BITMAP_SIZE).await {
            println!("failed to bitmap zone: {}, err: {}", zone_url, e);
            continue;
        };

        println!("{}/{}: {}", i + 1, zone_urls.len(), file_name);
    }

    fs::remove_dir_all(ZONE_DIR).await?;

    Ok(())
}

async fn get_access_token(client: &reqwest::Client, url: &str) -> Result<String, Box<dyn Error>> {
    let res = client
        .post(url)
        .json(&json!({
            "username": env::var("ICANN_USERNAME").unwrap(),
            "password": env::var("ICANN_PASSWORD").unwrap(),
        }))
        .send()
        .await?;

    let access_token = res.json::<Value>().await?["accessToken"]
        .as_str()
        .unwrap()
        .to_string();

    Ok(access_token)
}

async fn get_zone_urls(
    client: &reqwest::Client,
    url: &str,
    access_token: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    let zone_urls = res.json().await?;

    Ok(zone_urls)
}

async fn parse_zone_url(
    zone_url: &str,
    zone_dir: &str,
) -> Result<(String, String), Box<dyn Error>> {
    let zone_name = zone_url.split('/').last().unwrap();
    let file_name = zone_name.replace(".zone", ".txt").to_string();
    let file_path = format!("{}{}", zone_dir, file_name);

    // create dir if it doesn't exist
    if !Path::new(zone_dir).exists() {
        fs::create_dir_all(zone_dir).await?;
    }

    Ok((file_name, file_path))
}

async fn download_zone(
    client: &reqwest::Client,
    zone_url: &str,
    access_token: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let res = client
        .get(zone_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    // decompress
    let bytes = res.bytes().await?;
    let mut gz = GzDecoder::new(&bytes[..]);
    let mut s = String::new();
    gz.read_to_string(&mut s)?;

    // write to file
    let mut file = File::create(&file_path).await?;
    file.write_all(s.as_bytes()).await?;

    Ok(())
}

async fn bitmap_zone(
    file_path: &str,
    domains_file: &str,
    bitmap_size: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(domains_file)?;

    file.set_len(bitmap_size as u64)?;

    let mut mmap = unsafe { MmapOptions::new().map_mut(&file)? };

    // process the zone file
    let zone_file = File::open(file_path).await?;
    let reader = BufReader::new(zone_file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        if line.starts_with(';') || line.is_empty() {
            continue;
        }

        let mut domain = line.split_whitespace().next().unwrap_or("");
        if domain.ends_with('.') {
            domain = &domain[..domain.len() - 1];
        }
        if !domain.is_empty() {
            let index = Hash::domain_to_index(domain, bitmap_size);
            let byte_index = index / 8;
            let bit_index = index % 8;
            mmap[byte_index] |= 1 << bit_index;
        }
    }

    mmap.flush()?;

    Ok(())
}
