use crate::{error::AppError, Ctx};
use axum::{extract::State, Json};
use axum_extra::extract::Query;
use futures::future;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize)]
pub struct DnsLookupsParams {
    q: Vec<String>,
}

#[typeshare]
#[derive(Serialize)]
pub struct DnsLookupsRes {
    lookups: HashMap<String, bool>,
}

pub async fn mount(
    Query(params): Query<DnsLookupsParams>,
    State(ctx): State<Ctx>,
) -> Result<Json<DnsLookupsRes>, AppError> {
    let lookups = future::join_all(params.q.into_iter().map(|domain| async {
        let lookup = ctx.resolver.lookup_ip(domain.clone()).await;
        let available = match lookup {
            Ok(res) => res.iter().next().is_none(),
            Err(_) => true,
        };
        (domain, available)
    }))
    .await;

    let res = DnsLookupsRes {
        lookups: lookups.into_iter().collect(),
    };
    Ok(Json(res))
}
