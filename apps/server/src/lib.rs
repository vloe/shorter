use axum::{routing::get, Router};
use sh_core::api::mount;
use tower_service::Service;
use worker::*;

fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "shorter.dev server!" }))
        .merge(mount())
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    Ok(router().call(req).await?)
}
