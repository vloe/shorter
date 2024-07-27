use dotenv::dotenv;
use flate2::read::GzDecoder;
use memmap2::MmapOptions;
use serde_json::{json, Value};
use sh_crypto::hash::Hash;
use sh_domain::{domain_available::BITMAP_SIZE, tlds::TLDS};
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
    const ZONE_DIR: &str = "../apps/server/src/tmp/";
    const DOMAINS_FILE: &str = "../apps/server/data/domains.bin";

    let client = reqwest::Client::builder()
        .user_agent("curl/7.79.1")
        .build()?;

    let access_token = get_access_token(&client, CZDS_API_AUTH_URL).await?;
    let zone_urls = get_zone_urls(&client, CZDS_API_URL, &access_token).await?;

    let mut total_bits_used = 0;
    for (i, zone_url) in zone_urls.iter().enumerate() {
        let (zone, file_path) = parse_zone_url(zone_url, ZONE_DIR).await?;

        if TLDS.get(&zone.as_str()).is_none() {
            println!("invalid zone: {}", zone);
            continue;
        }

        if let Err(e) = download_zone(&client, zone_url, &access_token, &file_path).await {
            println!("failed to download zone: {}, err: {}", zone_url, e);
            continue;
        };
        match bitmap_zone(&file_path, DOMAINS_FILE, BITMAP_SIZE).await {
            Ok(bits_used) => {
                println!(
                    "{}/{}: {} ({} bits)",
                    i + 1,
                    zone_urls.len(),
                    zone,
                    bits_used
                );
                total_bits_used += bits_used;
            }
            Err(e) => {
                println!("failed to bitmap zone: {}, err: {}", zone, e);
                continue;
            }
        }
    }

    println!(
        "total bits: {}, bitmap bits: {}",
        total_bits_used,
        BITMAP_SIZE * 8
    );

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
    let data = zone_url.split('/').last().unwrap().replace(".zone", "");
    let zone = format!(".{}", data);
    let file_name = format!("{}.txt", data);
    let file_path = format!("{}{}", zone_dir, file_name);

    // create dir if it doesn't exist
    if !Path::new(zone_dir).exists() {
        fs::create_dir_all(zone_dir).await?;
    }

    Ok((zone, file_path))
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
) -> Result<i32, Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(domains_file)?;

    file.set_len(bitmap_size as u64)?;

    let mut mmap = unsafe { MmapOptions::new().map_mut(&file)? };

    let zone_file = File::open(file_path).await?;
    let reader = BufReader::new(zone_file);
    let mut lines = reader.lines();

    let mut bits_used = 0;
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
            mmap[index / 8] |= 1 << (7 - (index % 8));
            bits_used += 1;
        }
    }

    mmap.flush()?;

    Ok(bits_used)
}
