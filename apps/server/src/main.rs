use axum::{http::HeaderValue, routing::get, Router};
use hickory_resolver::TokioAsyncResolver;
use std::{error::Error, time::Duration};
use tower_http::cors::{Any, CorsLayer};

mod constants;
mod error;
mod models;
mod routes;

const MAX_AGE: u64 = 300; // 5 min
const ADDR: &str = "127.0.0.1:9000";

#[derive(Clone)]
struct Ctx {
    resolver: TokioAsyncResolver,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "https://shorter.dev".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers(Any)
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
        .layer(cors)
        .with_state(ctx);

    cfg_if::cfg_if! {
        if #[cfg(feature = "lambda")] {
            bind_lambda(app).await?;
        } else {
            bind(app).await?;
        }
    }

    Ok(())
}

#[cfg(feature = "lambda")]
async fn bind_lambda(app: Router) -> Result<(), Box<dyn Error>> {
    lambda_http::tracing::init_default_subscriber();
    lambda_http::run_with_streaming_response(app).await;
    Ok(())
}

#[cfg(not(feature = "lambda"))]
async fn bind(app: Router) -> Result<(), Box<dyn Error>> {
    println!("\nlistening on http://{}\n", ADDR);
    let listener = tokio::net::TcpListener::bind(&ADDR).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.unwrap();
            println!("\ngracefully shutting down...\n");
        })
        .await?;
    Ok(())
}
