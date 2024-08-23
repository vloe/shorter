use axum::routing::get;
use std::error::Error;

mod utils;

const ADDR: &str = "127.0.0.1:9000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = axum::Router::new()
        .route("/", get(|| async { "sh-server(:" }))
        .route("/health", get(|| async { "ok" }));

    println!("\nlistening on http://{}\n", ADDR);
    let listener = tokio::net::TcpListener::bind(&ADDR).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(utils::shutdown_signal())
        .await?;

    Ok(())
}
