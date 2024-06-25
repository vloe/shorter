use axum::{routing::get, Router};
use tower_service::Service;
use worker::*;

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();

    let kv = env.kv("tlds")?;

    let res = reqwest::get("https://data.iana.org/TLD/tlds-alpha-by-domain.txt")
        .await
        .unwrap();

    let text = res.text().await.unwrap();

    for line in text.lines() {
        if line.is_empty() || line.starts_with("#") || line.starts_with("XN--") {
            continue;
        }

        let tld = line.to_lowercase();
        kv.put(&tld, "test")?.execute().await?;
    }

    let app = Router::new()
        .route("/", get(|| async { "sh-tlds" }))
        .call(req)
        .await?;

    Ok(app)
}
