use axum::{
    error_handling::HandleErrorLayer,
    http::{header, HeaderValue, Method, StatusCode},
    routing::get,
    BoxError, Router,
};
use bitvec::prelude::*;
use hickory_resolver::TokioAsyncResolver;
use sh_core::api::{mount, Ctx};
use std::time::Duration;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::cors::CorsLayer;

mod utils;

const BIT_MAP: &[u8] = include_bytes!("../data/domains.bin");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    const MAX_AGE: u64 = 3600;
    const RATE_LIMIT_PERIOD: u64 = 1;
    const RATE_LIMIT_MAX_REQS: u64 = 10;
    const RATE_LIMIT_BUFFER: usize = 1000;
    const ADDR: &str = "127.0.0.1:9000";

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:3001".parse::<HeaderValue>().unwrap(),
            "http://localhost:3002".parse::<HeaderValue>().unwrap(),
            "https://shorter.dev".parse::<HeaderValue>().unwrap(),
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

    let ctx = get_ctx()?;

    let app = Router::new()
        .route("/", get(|| async { "sh-server" }))
        .merge(mount(ctx))
        .layer(cors)
        .layer(rate_limit);

    #[cfg(feature = "lambda")]
    run_prod(app).await?;

    #[cfg(not(feature = "lambda"))]
    run_dev(app, ADDR).await?;

    Ok(())
}

fn get_ctx() -> Result<Ctx, Box<dyn std::error::Error>> {
    let resolver = {
        let (cfg, opts) = hickory_resolver::system_conf::read_system_conf()?;
        TokioAsyncResolver::tokio(cfg, opts).into()
    };

    let domains = unsafe { BitSlice::from_slice_unchecked(BIT_MAP) };

    let ctx = Ctx { resolver, domains };

    Ok(ctx)
}

#[cfg(feature = "lambda")]
async fn run_prod(app: Router) -> Result<(), Box<dyn std::error::Error>> {
    use lambda_http::{run, tracing, Error};

    tracing::init_default_subscriber();
    run(app).await;

    Ok(())
}

#[cfg(not(feature = "lambda"))]
async fn run_dev(app: Router, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nlistening on http://{}\n", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(utils::shutdown_signal())
        .await?;

    Ok(())
}
