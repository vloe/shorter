use dotenv::dotenv;
use flate2::read::GzDecoder;
use futures::StreamExt;
use memmap2::{MmapMut, MmapOptions};
use serde_json::{json, Value};
use sh_core::domain::{domain_to_index, DOMAINS_BIT_SIZE, DOMAINS_BYTE_SIZE};
use std::{env, error::Error, io::Read, sync::Arc, time::Duration};
use tokio::{
    fs::{self, File, OpenOptions},
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    sync::Mutex,
    time::sleep,
};

const CZDS_API_URL: &str = "https://czds-api.icann.org/czds/downloads/links";
const CZDS_API_AUTH_URL: &str = "https://account-api.icann.org/api/authenticate";
const TMP_DIR: &str = "tmp/";
const DOMAINS_FILE: &str = "../apps/server/data/domains.bin";
const DOMAINS_DIR: &str = "../apps/server/data/domains/";
const CHUNK_SIZE: usize = 1024 * 1024; // 1 mb chunks
const MAX_RETRIES: u32 = 3;
const RETRY_DELAY: u64 = 5; // seconds

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let client = reqwest::Client::builder()
        .user_agent("curl/7.79.1")
        .build()?;

    let access_token = get_access_token(&client).await?;
    let zone_urls = get_zone_urls(&client, &access_token).await?;

    fs::create_dir_all(TMP_DIR).await?;
    fs::create_dir_all(DOMAINS_DIR).await?;

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(DOMAINS_FILE)
        .await?;

    file.set_len(DOMAINS_BYTE_SIZE as u64).await?;

    let mmap = unsafe { MmapOptions::new().map_mut(&file)? };
    let mmap = Arc::new(Mutex::new(mmap));

    let mut bits_used = 0;
    for (i, zone_url) in zone_urls.iter().enumerate() {
        let (name, file_path) = get_zone_data(zone_url).await?;

        println!("{}/{} {}:", i + 1, zone_urls.len(), name);

        match download_zone_with_retry(&client, &access_token, &zone_url, &file_path).await {
            Ok(_) => {
                println!("downloaded");
            }
            Err(e) => {
                println!("download error: {}", e);
                continue;
            }
        }

        match bitmap_zone(&file_path, Arc::clone(&mmap)).await {
            Ok(bits) => {
                println!("bitmaped");
                bits_used += bits;
            }
            Err(e) => {
                println!("bitmap error: {}", e);
                continue;
            }
        }

        if let Err(e) = fs::remove_file(&file_path).await {
            println!("error removing temporary file: {}", e);
        }
    }

    println!("used {}/{} bits", bits_used, DOMAINS_BIT_SIZE);

    fs::remove_dir_all(TMP_DIR).await?;

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
        .await?;

    let access_token = res.json::<Value>().await?["accessToken"]
        .as_str()
        .unwrap()
        .to_string();

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
        .await?;

    let zone_urls = res.json().await?;

    Ok(zone_urls)
}

async fn get_zone_data(zone_url: &str) -> Result<(String, String), Box<dyn Error>> {
    let name = zone_url.split('/').last().unwrap().replace(".zone", "");
    let file_path = format!("{}{}.txt", TMP_DIR, name);

    Ok((name, file_path))
}

async fn download_zone_with_retry(
    client: &reqwest::Client,
    access_token: &str,
    zone_url: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    for attempt in 1..=MAX_RETRIES {
        match download_zone(client, access_token, zone_url, file_path).await {
            Ok(_) => return Ok(()),
            Err(e) => {
                println!("download attempt {} failed: {}", attempt, e);
                if attempt < MAX_RETRIES {
                    println!("retrying in {} seconds...", RETRY_DELAY);
                    sleep(Duration::from_secs(RETRY_DELAY)).await;
                } else {
                    return Err(e.into());
                }
            }
        }
    }
    Err("max retries reached".into())
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
        .await?;

    let mut file = File::create(file_path).await?;
    let mut stream = res.bytes_stream();
    let mut compressed_data = Vec::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        compressed_data.extend_from_slice(&chunk);
    }

    let mut gz = GzDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    gz.read_to_end(&mut decompressed_data)?;

    file.write_all(&decompressed_data).await?;

    Ok(())
}

async fn bitmap_zone(file_path: &str, mmap: Arc<Mutex<MmapMut>>) -> Result<i32, Box<dyn Error>> {
    let file = File::open(file_path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut bits = 0;
    let mut buffer = Vec::new();

    while let Some(line) = lines.next_line().await? {
        if line.starts_with(';') || line.is_empty() {
            continue;
        }

        let mut domain = line.split_whitespace().next().unwrap_or("");
        if domain.ends_with('.') {
            domain = &domain[..domain.len() - 1];
        }

        if !domain.is_empty() {
            let index = domain_to_index(domain);
            buffer.push(index);
            bits += 1;
        }

        if buffer.len() >= CHUNK_SIZE {
            update_bitmap(&mmap, &buffer).await?;
            buffer.clear();
        }
    }

    if !buffer.is_empty() {
        update_bitmap(&mmap, &buffer).await?;
    }

    Ok(bits)
}

async fn update_bitmap(
    mmap: &Arc<Mutex<MmapMut>>,
    indices: &Vec<usize>,
) -> Result<(), Box<dyn Error>> {
    let mut mmap = mmap.lock().await;
    for &index in indices {
        let byte_index = index as usize / 8;
        let bit_index = index % 8;
        mmap[byte_index] |= 1 << bit_index;
    }
    Ok(())
}
