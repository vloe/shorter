use axum::{routing::get, Router};
use sh_core::api::mount;
use tower_service::Service;
use worker::*;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    let app = Router::new()
        .route("/", get(|| async { "sh-server" }))
        .merge(mount())
        .call(req)
        .await?;

    Ok(app)
}
