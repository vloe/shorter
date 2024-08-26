use scraper::{Html, Selector};
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
};

struct TldInfo {
    name: String,
    category: String,
    manager: String,
}

const IANA_TLDS_URL: &str = "https://www.iana.org/domains/root/db";
const TLD_INFO_FILE: &str = "../apps/server/src/constants/tld_info.rs";
const TMP_TLD_INFO_FILE: &str = "tmp/tld_info.rs";
const TMP_DIR: &str = "tmp/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .user_agent("curl/7.79.1")
        .build()?;

    fs::create_dir_all(TMP_DIR)?;

    let tld_info = get_iana_tld_info(&client).await?;
    write_tld_info(tld_info)?;

    fs::rename(TMP_TLD_INFO_FILE, TLD_INFO_FILE)?; // swap tmp file w the real one
    fs::remove_dir_all(TMP_DIR)?;

    Ok(())
}

async fn get_iana_tld_info(client: &reqwest::Client) -> Result<Vec<TldInfo>, Box<dyn Error>> {
    let res = client
        .get(IANA_TLDS_URL)
        .send()
        .await
        .map_err(|e| format!("failed to fetch data from {}: {}", IANA_TLDS_URL, e))?;

    if !res.status().is_success() {
        return Err(format!(
            "failed to fetch data from {} w status code: {}",
            IANA_TLDS_URL,
            res.status()
        )
        .into());
    }

    let text = res.text().await?;
    let doc = Html::parse_document(&text);
    let row_selector = Selector::parse("table#tld-table tbody tr")?;
    let cell_selector = Selector::parse("td")?;

    let mut tld_info = Vec::new();
    for row in doc.select(&row_selector) {
        let cells: Vec<_> = row.select(&cell_selector).collect();
        if cells.len() < 3 {
            continue;
        }

        let name = cells[0]
            .text()
            .collect::<String>()
            .trim()
            .to_lowercase()
            .replace(".", "");
        let category = cells[1].text().collect::<String>().trim().to_lowercase();
        let manager = cells[2]
            .text()
            .collect::<String>()
            .trim()
            .to_lowercase()
            .replace("not assigned", "")
            .replace('"', "");

        if name.is_empty() || category.is_empty() || manager.is_empty() {
            continue;
        }

        let tld = TldInfo {
            name,
            category,
            manager,
        };
        tld_info.push(tld);
    }

    if tld_info.is_empty() {
        return Err(
            "failed to get iana tlds, mby the website is down or changed structure?".into(),
        );
    }

    Ok(tld_info)
}

fn write_tld_info(tld_info: Vec<TldInfo>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(TMP_TLD_INFO_FILE)?;

    writeln!(file, "// don't edit this file, it's auto-generated")?;
    writeln!(file, "use phf::{{phf_map, Map}};")?;
    writeln!(file, "use serde::Serialize;")?;
    writeln!(file, "use std::clone::Clone;")?;
    writeln!(file, "use typeshare::typeshare;")?;
    writeln!(file)?;
    writeln!(file, "#[typeshare]")?;
    writeln!(file, "#[derive(Serialize, Clone)]")?;
    writeln!(file, "pub struct TldInfo {{")?;
    writeln!(file, "    pub name: &'static str,")?;
    writeln!(file, "    pub category: &'static str,")?;
    writeln!(file, "    pub manager: &'static str,")?;
    writeln!(file, "}}")?;
    writeln!(file)?;
    writeln!(file, "pub static TLD_INFO: Map<&str, Tld> = phf_map!(")?;

    for tld in tld_info {
        writeln!(
            file,
            "    \"{}\" => TldInfo {{ name: \"{}\", category: \"{}\", manager: \"{}\" }},",
            tld.name, tld.name, tld.category, tld.manager
        )?;
    }

    writeln!(file, ");")?;

    Ok(())
}
