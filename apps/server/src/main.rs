use axum::{
    error_handling::HandleErrorLayer,
    http::{header, HeaderValue, Method, StatusCode},
    routing::get,
    BoxError, Router,
};
use hickory_resolver::TokioAsyncResolver;
use lambda_http::{run, tracing, Error};
use sh_core::api::{mount, Ctx};
use std::time::Duration;
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:3001".parse::<HeaderValue>().unwrap(),
            "http://localhost:3002".parse::<HeaderValue>().unwrap(),
            "https://shorter.dev".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .max_age(Duration::from_secs(3600));

    let rate_limit = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|err: BoxError| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("internal error: {}", err),
            )
        }))
        .layer(BufferLayer::new(1000))
        .layer(RateLimitLayer::new(10, Duration::from_secs(1)));

    let resolver = {
        let (cfg, opts) = hickory_resolver::system_conf::read_system_conf()?;
        TokioAsyncResolver::tokio(cfg, opts).into()
    };

    let ctx = Ctx { resolver };

    let app = Router::new()
        .route("/", get(|| async { "sh-server" }))
        .merge(mount(ctx))
        .layer(cors)
        .layer(rate_limit);

    println!("\nlistening on: http://localhost:9000\n");
    run(app).await
}
