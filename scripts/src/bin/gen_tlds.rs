use scraper::{Html, Selector};
use std::{error::Error, fs::File, io::Write};

struct Tld {
    name: String,
    category: String,
    manager: String,
}

const IANA_URL: &str = "https://www.iana.org/domains/root/db";
const TLDS_FILE: &str = "../core/src/constants/tlds.rs";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .user_agent("curl/7.79.1")
        .build()?;

    let tlds = get_iana_tlds(&client).await?;
    write_tlds(tlds)?;

    Ok(())
}

async fn get_iana_tlds(client: &reqwest::Client) -> Result<Vec<Tld>, Box<dyn Error>> {
    let res = client.get(IANA_URL).send().await?;
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

fn write_tlds(tlds: Vec<Tld>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(TLDS_FILE)?;

    writeln!(file, "// do not edit this file, it's auto-generated")?;
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
