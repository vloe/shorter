use axum::{
    http::{header, HeaderValue, Method},
    routing::get,
    Router,
};
use hickory_resolver::TokioAsyncResolver;
use lambda_http::{run, tracing, Error};
use sh_core::api::{mount, Ctx};
use std::time::Duration;
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

    let resolver = {
        let (cfg, opts) = hickory_resolver::system_conf::read_system_conf()?;
        TokioAsyncResolver::tokio(cfg, opts).into()
    };

    let ctx = Ctx { resolver };

    let app = Router::new()
        .route("/", get(|| async { "sh-server" }))
        .merge(mount(ctx))
        .layer(cors);

    println!("\nlistening on: http://localhost:9000\n");
    run(app).await
}
