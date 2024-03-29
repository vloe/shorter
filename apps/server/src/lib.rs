use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, HeaderValue, Method, Response},
    routing::get,
    Router,
};
use sh_core::api::mount;
use tower_http::cors::CorsLayer;
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest, Result};

#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response<Body>> {
    console_error_panic_hook::set_once();

    let cors = CorsLayer::new()
        .allow_origin("https://shorter.dev".parse::<HeaderValue>().unwrap())
        .allow_headers(vec![CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS]);

    let app = Router::new()
        .route("/", get(|| async { "shorter.dev server!" }))
        .merge(mount())
        .layer(cors)
        .call(req)
        .await
        .unwrap();

    Ok(app)
}
