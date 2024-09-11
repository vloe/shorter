use crate::{error::AppError, Ctx};
use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize)]
pub struct DnsLookupParams {
    q: String,
}

#[typeshare]
#[derive(Serialize)]
pub struct DnsLookupRes {
    domain: String,
    available: bool,
}

pub async fn mount(
    Query(mut params): Query<DnsLookupParams>,
    State(ctx): State<Ctx>,
) -> Result<Json<DnsLookupRes>, AppError> {
    params.validate()?;

    let domain = params.q;
    let available = ctx
        .resolver
        .lookup_ip(&domain)
        .await
        .iter()
        .next()
        .is_none();

    let res = DnsLookupRes { domain, available };
    Ok(Json(res))
}
