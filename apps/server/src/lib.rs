use axum::{
    http::{header, HeaderValue, Method},
    routing::get,
    Router,
};
use sh_core::api::mount;
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tower_service::Service;
use worker::*;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://localhost:3001".parse::<HeaderValue>().unwrap(),
            "https://shorter.dev".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .max_age(Duration::from_secs(3600));

    let app = Router::new()
        .route("/", get(|| async { "sh-server" }))
        .merge(mount())
        .layer(cors)
        .call(req)
        .await?;

    Ok(app)
}
