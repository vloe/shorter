use dotenv::dotenv;
use flate2::read::GzDecoder;
use serde_json::{json, Value};
use std::{env, error::Error, io::prelude::*};
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    const CZDS_API_AUTH_URL: &str = "https://account-api.icann.org/api/authenticate";
    const CZDS_API_BASE_URL: &str = "https://czds-api.icann.org/czds/downloads/links";
    const ZONE_DIR: &str = "../crates/dommain/tmp/";
    const DOMAINS_FILE: &str = "../crates/domain/assets/domains.bin";

    let client = reqwest::Client::builder()
        .user_agent("curl/7.79.1")
        .build()?;

    let access_token = get_access_token(&client, CZDS_API_AUTH_URL).await?;
    let zone_urls = get_zone_urls(&client, CZDS_API_BASE_URL, &access_token).await?;

    for (i, zone_url) in zone_urls.iter().enumerate() {
        download_zone(&client, zone_url, &access_token, ZONE_DIR).await?;
        println!("{}/{}", i + 1, zone_urls.len());
    }

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

    if !res.status().is_success() {
        return Err(format!("failed to get access token from: {}", url).into());
    }

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

    if !res.status().is_success() {
        return Err(format!("failed to get zone files from: {}", url).into());
    }

    let zone_urls = res.json().await?;

    Ok(zone_urls)
}

async fn download_zone(
    client: &reqwest::Client,
    url: &str,
    access_token: &str,
    dir: &str,
) -> Result<(), Box<dyn Error>> {
    let res = client
        .get(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await?;

    if !res.status().is_success() {
        return Err(format!("failed to fetch zone url: {}", url).into());
    }

    let bytes = match res.bytes().await {
        Ok(b) => b,
        Err(e) => {
            return Err(format!("failed to fetch bytes from url: {}: {}", url, e).into());
        }
    };

    // decompress the data
    let mut gz = GzDecoder::new(&bytes[..]);
    let mut s = String::new();
    gz.read_to_string(&mut s)?;

    // write the data to file
    let file_name = url.split('/').last().unwrap().replace(".zone", ".txt");
    let file_path = format!("{}{}", dir, file_name);
    let mut file = File::create(&file_path).await?;
    file.write_all(s.as_bytes()).await?;

    Ok(())
}
