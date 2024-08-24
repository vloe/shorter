use axum::{
    http::{header, HeaderValue, Method},
    routing::get,
    Router,
};
use std::{error::Error, time::Duration};
use tower_http::cors::CorsLayer;

mod utils;

const MAX_AGE: u64 = 3600; // 1 hour
const ADDR: &str = "127.0.0.1:9000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

    let app = Router::new()
        .route("/", get(|| async { "sh-server(:" }))
        .route("/health", get(|| async { "ok" }))
        .layer(cors);

    #[cfg(feature = "lambda")]
    run_lambda(app).await?;

    #[cfg(not(feature = "lambda"))]
    run_dev(app).await?;

    Ok(())
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
    println!("\nlistening on http://{}\n", ADDR);
    let listener = tokio::net::TcpListener::bind(&ADDR).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(utils::shutdown_signal())
        .await?;

    Ok(())
}
