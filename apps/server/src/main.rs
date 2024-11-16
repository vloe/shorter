use axum::{
    http::{
        header::{ACCEPT, CACHE_CONTROL, CONTENT_TYPE},
        HeaderValue, Method,
    },
    routing::get,
    Router,
};
use hickory_resolver::TokioAsyncResolver;
use std::{error::Error, time::Duration};
use tower_http::cors::CorsLayer;

mod constants;
mod error;
mod routes;
mod utils;

const MAX_AGE: u64 = 300; // 5 min
const ADDR: &str = "127.0.0.1:9000";

#[derive(Clone)]
pub struct Ctx {
    resolver: TokioAsyncResolver,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "https://shorter.dev".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([ACCEPT, CACHE_CONTROL, CONTENT_TYPE])
        .max_age(Duration::from_secs(MAX_AGE));

    let resolver = {
        let (cfg, opts) = hickory_resolver::system_conf::read_system_conf()?;
        TokioAsyncResolver::tokio(cfg, opts)
    };
    let ctx = Ctx { resolver };

    let app = Router::new()
        .route("/", get(|| async { "sh-server(:" }))
        .route("/health", get(|| async { "ok" }))
        .route("/search", get(routes::search::mount))
        .route("/dns-lookup", get(routes::dns_lookup::mount))
        .layer(cors)
        .with_state(ctx);

    println!("\nlistening on http://{}\n", ADDR);

    #[cfg(feature = "lambda")]
    {
        lambda_http::tracing::init_default_subscriber();
        lambda_http::run(app).await;
    }

    #[cfg(not(feature = "lambda"))]
    {
        let listener = tokio::net::TcpListener::bind(&ADDR).await?;
        axum::serve(listener, app)
            .with_graceful_shutdown(utils::axum_shutdown_signal())
            .await?;
    }

    Ok(())
}
