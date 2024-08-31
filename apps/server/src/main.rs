use axum::{routing::get, Router};
use dotenv::dotenv;
use std::error::Error;

mod config;

const MAX_AGE: u64 = 3600;
const ADDR: &str = "127.0.0.1:9000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let cors = config::cors(MAX_AGE);
    let ctx = config::ctx();

    let app = Router::new()
        .route("/", get(|| async { "sh-server(:" }))
        .route("/health", get(|| async { "ok" }))
        .layer(cors)
        .with_state(ctx);

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
        .with_graceful_shutdown(config::shutdown_signal())
        .await?;

    Ok(())
}
