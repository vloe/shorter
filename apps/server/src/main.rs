use axum::{
    error_handling::HandleErrorLayer,
    http::{header, HeaderValue, Method, StatusCode},
    routing::get,
    BoxError, Router,
};
use memmap2::MmapOptions;
use sh_core::api::{mount, Ctx};
use std::{error::Error, sync::Arc, time::Duration};
use tokio::fs::File;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::cors::CorsLayer;

mod utils;

const MAX_AGE: u64 = 3600;
const RATE_LIMIT_PERIOD: u64 = 1;
const RATE_LIMIT_MAX_REQS: u64 = 10;
const RATE_LIMIT_BUFFER: usize = 1000;
const DOMAINS_FILE: &str = "data/domains.bin";
const DOMAINS_FILE_PROD: &str = "apps/server/data/domains.bin";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:3001".parse::<HeaderValue>().unwrap(),
            "http://localhost:3002".parse::<HeaderValue>().unwrap(),
            "https://shorter.dev".parse::<HeaderValue>().unwrap(),
            "https://app.shorter.dev".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .max_age(Duration::from_secs(MAX_AGE));

    let rate_limit = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|err: BoxError| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("internal error: {}", err),
            )
        }))
        .layer(BufferLayer::new(RATE_LIMIT_BUFFER))
        .layer(RateLimitLayer::new(
            RATE_LIMIT_MAX_REQS,
            Duration::from_secs(RATE_LIMIT_PERIOD),
        ));

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .merge(mount(ctx().await?))
        .layer(cors)
        .layer(rate_limit);

    #[cfg(feature = "lambda")]
    run_lambda(app).await?;

    #[cfg(not(feature = "lambda"))]
    run_dev(app).await?;

    Ok(())
}

async fn ctx() -> Result<Ctx, Box<dyn Error>> {
    let domains = {
        let file = match File::open(DOMAINS_FILE_PROD).await {
            Ok(file) => file,
            Err(_) => File::open(DOMAINS_FILE).await?,
        };
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        Arc::new(mmap)
    };

    let ctx = Ctx { domains };

    Ok(ctx)
}

#[cfg(feature = "lambda")]
async fn run_lambda(app: Router) -> Result<(), Box<dyn Error>> {
    use lambda_http::{run, tracing};

    tracing::init_default_subscriber();
    run(app).await;

    Ok(())
}

#[cfg(not(feature = "lambda"))]
async fn run_dev(app: Router) -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:9000";
    println!("\nlistening on http://{}\n", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(utils::shutdown_signal())
        .await?;

    Ok(())
}
