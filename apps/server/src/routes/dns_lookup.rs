use crate::{error::AppError, Ctx};
use axum::{extract::State, Json};
use axum_extra::extract::Query;
use futures::future;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize)]
pub struct DnsLookupParams {
    q: Vec<String>,
}

#[typeshare]
#[derive(Serialize)]
pub struct DnsLookupRes {
    lookup: HashMap<String, bool>,
}

pub async fn mount(
    Query(params): Query<DnsLookupParams>,
    State(ctx): State<Ctx>,
) -> Result<Json<DnsLookupRes>, AppError> {
    let lookup = future::join_all(params.q.into_iter().map(|domain| async {
        let available = match ctx.resolver.lookup_ip(domain.clone()).await {
            Ok(res) => res.iter().next().is_some(),
            Err(_) => false,
        };
        (domain, available)
    }))
    .await;

    let res = DnsLookupRes {
        lookup: lookup.into_iter().collect(),
    };
    Ok(Json(res))
}
