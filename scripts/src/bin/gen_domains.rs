use dotenv::dotenv;
use flate2::read::GzDecoder;
use futures::StreamExt;
use memmap2::{MmapMut, MmapOptions};
use serde_json::{json, Value};
use sh_core::domain::{domain_to_index, DOMAINS_BIT_SIZE, DOMAINS_BYTE_SIZE};
use std::{
    env,
    error::Error,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader},
    path::Path,
};
use tokio::io::AsyncWriteExt;
use tokio::{
    fs::File as TokioFile,
    time::{sleep, Duration},
};

const CZDS_API_URL: &str = "https://czds-api.icann.org/czds/downloads/links";
const CZDS_API_AUTH_URL: &str = "https://account-api.icann.org/api/authenticate";
const DOMAINS_FILE: &str = "../apps/server/data/domains.bin";
const TMP_DOMAINS_FILE: &str = "tmp/domains.bin";
const TMP_DIR: &str = "tmp/";
const MAX_RETRIES: usize = 3;
const RETRY_DELAY: u64 = 5; // secs

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let client = reqwest::Client::builder()
        .user_agent("curl/7.79.1")
        .build()?;

    let access_token = get_access_token(&client).await?;
    let zone_urls = get_zone_urls(&client, &access_token).await?;

    fs::create_dir_all(TMP_DIR)?;

    let mut mmap = create_bitmap()?;
    let mut total_bits = 0;
    for (i, zone_url) in zone_urls.iter().enumerate() {
        let file_name = zone_url.split('/').last().unwrap();
        let file_path = format!("{}{}", TMP_DIR, file_name);

        println!("{}/{} {}:", i + 1, zone_urls.len(), file_name);

        for _ in 0..MAX_RETRIES {
            match download_zone(&client, &access_token, zone_url, &file_path).await {
                Ok(_) => {
                    println!("downloaded");
                    break;
                }
                Err(e) => {
                    println!("failed to download: {}", e);
                    sleep(Duration::from_secs(RETRY_DELAY)).await;
                }
            }
        }

        // skip if download failed
        if !Path::new(&file_path).exists() {
            continue;
        }

        for _ in 0..MAX_RETRIES {
            match bitmap_zone(&file_path, &mut mmap) {
                Ok(bits) => {
                    total_bits += bits;
                    println!("bitmaped");
                    break;
                }
                Err(e) => {
                    println!("failed to bitmap: {}", e);
                    sleep(Duration::from_secs(RETRY_DELAY)).await;
                }
            }
        }

        fs::remove_file(file_path)?;
    }

    println!("used {}/{} bits", total_bits, DOMAINS_BIT_SIZE);

    fs::rename(TMP_DOMAINS_FILE, DOMAINS_FILE)?; // swap tmp file w the real one
    fs::remove_dir_all(TMP_DIR)?;

    Ok(())
}

async fn get_access_token(client: &reqwest::Client) -> Result<String, Box<dyn Error>> {
    let res = client
        .post(CZDS_API_AUTH_URL)
        .json(&json!({
            "username": env::var("ICANN_USERNAME").unwrap(),
            "password": env::var("ICANN_PASSWORD").unwrap(),
        }))
        .send()
        .await
        .map_err(|e| format!("failed to fetch data from {}: {}", CZDS_API_AUTH_URL, e))?;

    if !res.status().is_success() {
        return Err(format!(
            "failed to fetch data from {} w status code: {}",
            CZDS_API_AUTH_URL,
            res.status()
        )
        .into());
    }

    let data = res
        .json::<Value>()
        .await
        .map_err(|e| format!("failed to parse json: {}", e))?;

    let access_token = data["accessToken"].as_str().unwrap().to_string();
    if access_token.is_empty() {
        return Err("failed to get access token, mby the res structure has changed?".into());
    }

    Ok(access_token)
}

async fn get_zone_urls(
    client: &reqwest::Client,
    access_token: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let res = client
        .get(CZDS_API_URL)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|e| format!("failed to fetch data from {}: {}", CZDS_API_URL, e))?;

    if !res.status().is_success() {
        return Err(format!(
            "failed to fetch data from {} w status code: {}",
            CZDS_API_URL,
            res.status()
        )
        .into());
    }

    let data = res
        .json::<Value>()
        .await
        .map_err(|e| format!("failed to parse json: {}", e))?;

    let zone_urls: Vec<String> = data
        .as_array()
        .ok_or("res is not an array")?
        .iter()
        .filter_map(|url| url.as_str().map(String::from))
        .collect();

    if zone_urls.is_empty() {
        return Err("failed to get zone urls, mby the res structure has changed?".into());
    }

    Ok(zone_urls)
}

fn create_bitmap() -> Result<MmapMut, Box<dyn Error>> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(TMP_DOMAINS_FILE)?;

    file.set_len(DOMAINS_BYTE_SIZE as u64)?;

    let mmap = unsafe { MmapOptions::new().map_mut(&file)? };
    Ok(mmap)
}

async fn download_zone(
    client: &reqwest::Client,
    access_token: &str,
    zone_url: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let res = client
        .get(zone_url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|e| format!("failed to fetch data from {}: {}", zone_url, e))?;

    if !res.status().is_success() {
        return Err(format!(
            "failed to fetch data from {} w status code: {}",
            zone_url,
            res.status()
        )
        .into());
    }

    let mut file = TokioFile::create(file_path).await?;
    let mut stream = res.bytes_stream();
    while let Some(chunk_res) = stream.next().await {
        let chunk = chunk_res?;
        file.write_all(&chunk).await?;
    }

    Ok(())
}

fn bitmap_zone(file_path: &str, mmap: &mut MmapMut) -> Result<usize, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let gz = GzDecoder::new(file);
    let reader = BufReader::new(gz);

    let mut bits = 0;
    for line in reader.lines() {
        let line = line?;
        if line.starts_with(';') || line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let domain = parts[0].trim_end_matches('.');
        if !domain.is_empty() {
            let index = domain_to_index(domain);
            let byte_index = index / 8;
            let bit_index = index % 8;
            mmap[byte_index] |= 1 << bit_index;
            bits += 1;
        }
    }

    Ok(bits)
}
