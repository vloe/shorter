use reqwest;
use scraper::{Html, Selector};
use std::{collections::HashMap, error::Error, fs::File, io::Write};

struct Tld {
    name: String,
    category: String,
    manager: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const IANA_URL: &str = "https://www.iana.org/domains/root/db";
    const SERVER_LIST_URLS: [&str; 2] = [
        "https://raw.githubusercontent.com/rfc1036/whois/next/new_gtlds_list",
        "https://raw.githubusercontent.com/rfc1036/whois/next/tld_serv_list",
    ];
    const TLDS_FILE: &str = "../crates/domain/src/tlds.rs";

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;

    let mut tlds = get_iana_tlds(&client, IANA_URL).await?;
    let tld_servers = get_tld_servers(&client, SERVER_LIST_URLS).await?;
    tlds.retain(|tld| tld_servers.contains_key(&tld.name)); // remove tlds with no whois server

    write_tlds(tlds, TLDS_FILE)?;

    Ok(())
}

async fn get_iana_tlds(client: &reqwest::Client, url: &str) -> Result<Vec<Tld>, Box<dyn Error>> {
    let res = client.get(url).send().await?;

    if !res.status().is_success() {
        return Err(format!("failed to fetch iana tlds from: {}", url).into());
    }

    let text = res.text().await?;
    let doc = Html::parse_document(&text);
    let row_selector = Selector::parse("table#tld-table tbody tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let mut tlds = Vec::new();
    for row in doc.select(&row_selector) {
        let cells: Vec<_> = row.select(&cell_selector).collect();
        if cells.len() >= 3 {
            let name = cells[0].text().collect::<String>().trim().to_lowercase();
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

            let tld = Tld {
                name,
                category,
                manager,
            };

            tlds.push(tld);
        }
    }

    Ok(tlds)
}

async fn get_tld_servers(
    client: &reqwest::Client,
    urls: [&str; 2],
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut tld_servers: HashMap<String, String> = HashMap::new();

    for url in urls {
        let res = client.get(url).send().await?;

        if !res.status().is_success() {
            return Err(format!("failed to fetch tld servers from: {}", url).into());
        }

        let text = res.text().await?;
        for line in text.lines() {
            if line.is_empty() || line.starts_with('#') || line.contains("NONE") {
                continue;
            }

            let mut tld = line.split_whitespace().next().unwrap().to_string();
            if !tld.contains(".") {
                tld = format!(".{}", tld);
            }

            if !tld_servers.contains_key(&tld) {
                tld_servers.insert(tld.clone(), tld.clone());
            }
        }
    }

    Ok(tld_servers)
}

fn write_tlds(tlds: Vec<Tld>, file_path: &str) -> Result<(), Box<dyn Error>> {
    const WARNING_MSG: &str = "don't edit this file, it's auto-generated.";

    let mut file = File::create(file_path)?;
    writeln!(file, "// {}", WARNING_MSG)?;
    writeln!(file, "use phf::{{phf_map, Map}};")?;
    writeln!(file, "use serde::Serialize;")?;
    writeln!(file, "use std::clone::Clone;")?;
    writeln!(file, "use typeshare::typeshare;")?;
    writeln!(file)?;
    writeln!(file, "#[typeshare]")?;
    writeln!(file, "#[derive(Serialize, Clone)]")?;
    writeln!(file, "pub struct Tld {{")?;
    writeln!(file, "    pub name: &'static str,")?;
    writeln!(file, "    pub category: &'static str,")?;
    writeln!(file, "    pub manager: &'static str,")?;
    writeln!(file, "}}")?;
    writeln!(file)?;
    writeln!(file, "pub static TLDS: Map<&str, Tld> = phf_map!(")?;

    for tld in tlds {
        writeln!(
            file,
            "    \"{}\" => Tld {{ name: \"{}\", category: \"{}\", manager: \"{}\" }},",
            tld.name, tld.name, tld.category, tld.manager
        )?;
    }

    writeln!(file, ");")?;

    Ok(())
}
