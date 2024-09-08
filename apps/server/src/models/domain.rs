use crate::constants::tld_info::{TldInfo, TLD_INFO};
use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub name: String,
    pub sld: String,
    pub tld: String,
    pub tld_with_dot: String,
    pub tld_info: TldInfo,
    pub is_registered: bool,
}

impl Domain {
    pub fn new(domain: &str) -> Self {
        let name = domain.to_string();
        let (sld, tld, tld_with_dot) = Self::get_domain_parts(&name);
        let tld_info = Self::get_tld_info(&tld);
        let is_registered = false;

        Domain {
            name,
            sld,
            tld,
            tld_with_dot,
            tld_info,
            is_registered,
        }
    }

    fn get_domain_parts(domain: &str) -> (String, String, String) {
        let parts: Vec<&str> = domain.split('.').collect();

        let sld = parts[0].to_string();
        let tld = parts[1].to_string();
        let tld_with_dot = format!(".{}", tld);

        (sld, tld, tld_with_dot)
    }

    fn get_tld_info(tld: &str) -> TldInfo {
        TLD_INFO.get(tld).unwrap().clone()
    }
}
