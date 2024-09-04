use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize)]
pub struct SearchParams {
    q: String,
}

pub async fn mount(Query(params): Query<SearchParams>) -> impl IntoResponse {
    "test".into_response()
}
