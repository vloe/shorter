use reqwest;
use scraper::{Html, Selector};
use std::{collections::HashMap, error::Error, fs::File, io::Write};

#[derive(Debug)]
struct Tld {
    name: String,
    category: String,
    manager: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut tlds = Vec::new();
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .build()?;

    // get tlds, categories and managers.
    let res = client
        .get("https://www.iana.org/domains/root/db")
        .send()
        .await?;
    let text = res.text().await?;

    let doc = Html::parse_document(&text);
    let row_selector = Selector::parse("table#tld-table tbody tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

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

            tlds.push(Tld {
                name,
                category,
                manager,
            });
        }
    }

    // get tlds with whois server
    let mut tlds_with_server: HashMap<String, String> = HashMap::new();

    let res = client
        .get("https://raw.githubusercontent.com/rfc1036/whois/next/new_gtlds_list")
        .send()
        .await?;
    let text = res.text().await?;

    for line in text.lines() {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let tld = format!(".{}", line.trim());
        tlds_with_server.insert(tld.clone(), tld.clone());
    }

    let res = client
        .get("https://raw.githubusercontent.com/rfc1036/whois/next/tld_serv_list")
        .send()
        .await?;
    let text = res.text().await?;

    for line in text.lines() {
        if line.is_empty() || line.starts_with('#') || line.contains("NONE") {
            continue;
        }

        let tld = line.split_whitespace().next().unwrap().to_string();
        if !tlds_with_server.contains_key(&tld) {
            tlds_with_server.insert(tld.clone(), tld.clone());
        }
    }

    // remove tlds with no whois server
    tlds.retain(|tld| tlds_with_server.contains_key(&tld.name));

    // finally write the tlds to file
    let mut file = File::create("../crates/domain/tlds.rs")?;
    writeln!(file, "// don't edit this file, it's auto-generated.")?;
    writeln!(file, "use phf::{{phf_map, Map}};")?;
    writeln!(file, "use serde::Serialize;")?;
    writeln!(file, "use std::clone::Clone;")?;
    writeln!(file, "use typeshare::typeshare;")?;
    writeln!(file)?;
    writeln!(file, "#[typeshare]")?;
    writeln!(file, "#[derive(Serialize, Clone)]")?;
    writeln!(file, "pub(crate) struct Tld {{")?;
    writeln!(file, "    pub(crate) name: &'static str,")?;
    writeln!(file, "    pub(crate) category: &'static str,")?;
    writeln!(file, "    pub(crate) manager: &'static str,")?;
    writeln!(file, "}}")?;
    writeln!(file)?;
    writeln!(file, "pub(crate) static TLDS: Map<&str, Tld> = phf_map!(")?;

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
