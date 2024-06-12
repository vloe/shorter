use axum::{
    http::{header::CONTENT_TYPE, HeaderValue, Method},
    routing::get,
    Router,
};
use sh_core::api::mount;
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
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_headers([CONTENT_TYPE])
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/", get(|| async { "sh-server" }))
        .merge(mount())
        .layer(cors)
        .call(req)
        .await?;

    Ok(app)
}
