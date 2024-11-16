use axum::{http::HeaderValue, routing::get, Router};
use hickory_resolver::TokioAsyncResolver;
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};
use tower_http::cors::{Any, CorsLayer};

mod constants;
mod error;
mod routes;

const MAX_AGE: u64 = 300; // 5 min
const PORT: u16 = 9000;

#[derive(Clone)]
pub struct Ctx {
    resolver: TokioAsyncResolver,
}

#[tokio::main]
async fn main() -> Result<(), error::AppError> {
    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "https://shorter.dev".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(Duration::from_secs(MAX_AGE));

    let resolver = {
        let (cfg, opts) = hickory_resolver::system_conf::read_system_conf()
            .map_err(|e| error::AppError::Internal(e.to_string()))?;
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

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), PORT);
    println!("\nlistening on http://{}\n", addr);

    cfg_if::cfg_if! {
        if #[cfg(feature = "lambda")] {
            bind_lambda(app).await?;
        } else {
            bind(app, addr).await?;
        }
    }

    Ok(())
}

#[cfg(feature = "lambda")]
async fn bind_lambda(app: Router) -> Result<(), error::AppError> {
    lambda_http::tracing::init_default_subscriber();
    lambda_http::run(app).await.expect("failed start lambda");
    Ok(())
}

#[cfg(not(feature = "lambda"))]
async fn bind(app: Router, addr: SocketAddr) -> Result<(), error::AppError> {
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| error::AppError::Internal(e.to_string()))?;
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install ctrl+c handler");
            println!("\nshutting down gracefully...\n");
        })
        .await
        .map_err(|e| error::AppError::Internal(e.to_string()))?;
    Ok(())
}
