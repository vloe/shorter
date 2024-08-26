use axum::http::{header, HeaderValue, Method};
use sh_core::routes::Ctx;
use std::{sync::Arc, time::Duration};
use tokio::signal;
use tower_http::cors::CorsLayer;

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

pub fn ctx() -> Ctx {
    let msg = Arc::new("test".to_string());
    Ctx { msg }
}

pub(crate) async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to install ctrl+c handler");

    println!("\ngracefully shutting down...\n");
}
