use axum::http::{header, HeaderValue, Method};
use std::{sync::Arc, time::Duration};
use tokio::signal;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct Ctx {
    pub msg: Arc<String>,
}

pub fn ctx() -> Ctx {
    let msg = Arc::new(String::from("Hello, World!"));
    Ctx { msg }
}

pub fn cors(max_age: u64) -> CorsLayer {
    CorsLayer::new()
        .allow_origin([
            "http://localhost:3001".parse::<HeaderValue>().unwrap(),
            "https://dashboard.shorter.dev"
                .parse::<HeaderValue>()
                .unwrap(),
        ])
        .allow_methods([Method::GET])
        .allow_headers([header::CONTENT_TYPE])
        .max_age(Duration::from_secs(max_age))
}

pub async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to install ctrl+c handler");

    println!("\ngracefully shutting down...\n");
}
