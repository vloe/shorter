use crate::{constants::tlds::TLDS, error::AppError, routes::search::MAX_DOMAIN, Ctx};
use axum::{
    extract::{Query, State},
    Json,
};
use hickory_resolver::proto::rr::RecordType;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize)]
pub struct DnsLookupParams {
    q: String,
}

impl DnsLookupParams {
    pub fn validate(&self) -> Result<(), AppError> {
        let trimmed = self.q.trim();
        if trimmed.is_empty() {
            return Err(AppError::IsEmpty("domain"));
        }
        if trimmed.len() > (MAX_DOMAIN + 4) {
            return Err(AppError::TooLong("domain", MAX_DOMAIN));
        }
        Ok(())
    }
}

#[typeshare]
#[derive(Serialize)]
pub struct DnsLookupRes {
    buyable: bool,
}

pub async fn mount(
    Query(params): Query<DnsLookupParams>,
    State(ctx): State<Ctx>,
) -> Result<Json<DnsLookupRes>, AppError> {
    params.validate()?;

    let domain = params.q.trim().to_lowercase();

    // first check if tld is buyable
    let parts: Vec<&str> = domain.split('.').collect();
    if let Some(tld) = TLDS.get(parts[1]) {
        if !tld.buyable {
            let res = DnsLookupRes { buyable: false };
            return Ok(Json(res));
        }
    };

    let record_types = vec![
        RecordType::A,
        RecordType::NS,
        RecordType::SOA,
        RecordType::MX,
    ];

    for record_type in record_types {
        match ctx.resolver.lookup(&domain, record_type).await {
            Ok(res) if !res.records().is_empty() => {
                let res = DnsLookupRes { buyable: false };
                return Ok(Json(res));
            }
            _ => continue,
        }
    }

    let res = DnsLookupRes { buyable: true };
    Ok(Json(res))
}
