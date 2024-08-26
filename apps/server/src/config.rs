use axum::http::{header, HeaderValue, Method};
use sh_core::routes::Ctx;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::{sync::Arc, time::Duration};
use tokio::signal;
use tower_http::cors::CorsLayer;

pub fn cors(max_age: u64) -> CorsLayer {
    CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:3001".parse::<HeaderValue>().unwrap(),
            "http://localhost:3002".parse::<HeaderValue>().unwrap(),
            "https://shorter.dev".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .max_age(Duration::from_secs(max_age))
}

pub async fn ctx() -> Ctx {
    let db = {
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .expect("failed to create pool");
        Arc::new(pool)
    };

    Ctx { db }
}

pub(crate) async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("failed to install ctrl+c handler");

    println!("\ngracefully shutting down...\n");
}
