use axum::{
    http::{
        header::{CACHE_CONTROL, CONTENT_TYPE},
        HeaderValue, Method,
    },
    middleware::{self, Next},
    routing::get,
    Router,
};
use sh_core::api::mount;
use tower_http::cors::CorsLayer;
use tower_service::Service;
use worker::*;

async fn cache_control(req: axum::extract::Request, next: Next) -> axum::response::Response {
    let mut res = next.run(req).await;
    res.headers_mut()
        .insert(CACHE_CONTROL, "max-age=600".parse::<HeaderValue>().unwrap());
    res
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "https://shorter.dev".parse::<HeaderValue>().unwrap(),
        ])
        .allow_headers([CONTENT_TYPE])
        .allow_methods([Method::GET]);

    let app = Router::new()
        .route("/", get(|| async { "sh-server" }))
        .merge(mount())
        .layer(cors)
        .layer(middleware::from_fn(cache_control))
        .call(req)
        .await?;

    Ok(app)
}
