use lambda_http::{run, tracing, Error};
use tower_http::cors::CorsLayer;

use axum::{
    http::{HeaderValue, Method},
    routing::{get, post},
    Router,
};

use std::time::Duration;

const MAX_AGE: u64 = 300; // 5 min
const ADDR: &str = "127.0.0.1:9000";

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "https://shorter.dev".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST])
        .max_age(Duration::from_secs(MAX_AGE));

    let app = Router::new()
        .route("/", get(|| async { "sh-server(:" }))
        .route("/health", get(|| async { "ok" }))
        .layer(cors);

    run(app).await
}
