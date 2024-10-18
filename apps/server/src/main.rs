use lambda_http::{run, tracing, Error};

use axum::{Router, routing::get};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let app = Router::new()
        .route("/", get(|| async { "sh-server(:" }))
        .route("/health", get(|| async { "ok" }));

    run(app).await
}
