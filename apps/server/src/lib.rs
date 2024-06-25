use axum::{
    body::Body,
    extract::Request,
    http::{header, HeaderValue, Method, Response},
    middleware,
    routing::get,
    Router,
};
use sh_core::api::mount;
use tower_http::cors::CorsLayer;
use tower_service::Service;
use worker::{event, Context, Env, Error, HttpRequest};

async fn cache(req: Request, next: middleware::Next) -> axum::response::Response {
    let mut res = next.run(req).await;
    res.headers_mut().insert(
        header::CACHE_CONTROL,
        "max-age=600".parse::<HeaderValue>().unwrap(),
    );
    res
}

#[event(fetch)]
async fn fetch(req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response<Body>, Error> {
    console_error_panic_hook::set_once();

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:5173".parse::<HeaderValue>().unwrap(),
            "https://shorter.dev".parse::<HeaderValue>().unwrap(),
        ])
        .allow_headers([header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::DELETE]);

    let app = Router::new()
        .route("/", get(|| async { "sh-server" }))
        .merge(mount())
        .layer(cors)
        .layer(middleware::from_fn(cache))
        .call(req)
        .await?;

    Ok(app)
}
