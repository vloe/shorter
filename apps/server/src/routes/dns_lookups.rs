use crate::{error::AppError, Ctx};
use axum::{extract::State, Json};
use axum_extra::extract::Query;
use futures::future;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize)]
pub struct DnsLookupsParams {
    q: Vec<String>,
}

#[typeshare]
#[derive(Serialize)]
pub struct Lookup {
    domain: String,
    available: bool,
}

#[typeshare]
#[derive(Serialize)]
pub struct DnsLookupsRes {
    lookups: Vec<Lookup>,
}

pub async fn mount(
    Query(params): Query<DnsLookupsParams>,
    State(ctx): State<Ctx>,
) -> Result<Json<DnsLookupsRes>, AppError> {
    let lookups = future::join_all(params.q.into_iter().map(|domain| async {
        let lookup = ctx.resolver.lookup_ip(domain.clone()).await;
        let available = match lookup {
            Ok(result) => result.iter().next().is_none(),
            Err(_) => true,
        };
        Lookup { domain, available }
    }))
    .await;

    let res = DnsLookupsRes { lookups };
    Ok(Json(res))
}
