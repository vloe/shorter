use axum::http::{header, HeaderValue, Method};
use memmap2::{Mmap, MmapOptions};
use std::{error::Error, fs::File, sync::Arc, time::Duration};
use tokio::signal;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub(crate) struct Ctx {
    pub(crate) domains: Arc<Mmap>,
}

pub fn cors(max_age: u64) -> CorsLayer {
    CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:3001".parse::<HeaderValue>().unwrap(),
            "http://localhost:3002".parse::<HeaderValue>().unwrap(),
            "https://shorter.dev".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .max_age(Duration::from_secs(max_age))
}

pub fn ctx(domains_file: &str) -> Result<Ctx, Box<dyn Error>> {
    // print stuff files in the domains dir
    let dir = std::fs::read_dir(domains_file)?;
    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("file: {:?}", path);
        }
    }

    // also print deaper stuff
    let dir = std::fs::read_dir(domains_file)?;
    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("file: {:?}", path);
        } else if path.is_dir() {
            println!("dir: {:?}", path);
        }
    }

    let domains = {
        let file = File::open(domains_file)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        Arc::new(mmap)
    };

    let ctx = Ctx { domains };
    Ok(ctx)
}

pub(crate) async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to install ctrl+c handler");

    println!("\ngracefully shutting down...\n");
}
