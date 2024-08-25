use crate::constants::tlds::{Tld, TLDS};
use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize)]
pub(crate) struct Domain {
    name: String,
    tld: Tld,
    available: bool,
}

impl Domain {
    pub(crate) fn new(name: &str) -> Self {
        let name = name.to_string();
        let tld = Self::get_tld(&name);
        let available = Self::is_available(&name);

        Self {
            name,
            tld,
            available,
        }
    }

    fn get_tld(name: &str) -> Tld {
        let (_, tld) = get_sld_tld(name);
        TLDS.get(&tld).unwrap().clone()
    }

    fn is_available(name: &str) -> bool {
        false
    }
}

pub(crate) fn get_sld_tld(domain: &str) -> (String, String) {
    let parts: Vec<&str> = domain.split('.').collect();
    (parts[0].to_string(), parts[1].to_string())
}
