import { DOMParser } from "xmldom"
import fs from "fs"

const writeTldInfo = async () => {
	const tldInfo = []

	// get tlds, categories, and managers
	const res = await fetch("https://www.iana.org/domains/root/db")
	const text = await res.text()
	const doc = new DOMParser().parseFromString(text, "text/html")

	const table = doc.getElementsByTagName("table")[0]
	const rows = table.getElementsByTagName("tr")

	for (let i = 1; i < rows.length; i++) {
		const cells = rows[i].getElementsByTagName("td")
		const tld = cells[0].textContent.trim()
		const type = cells[1].textContent.trim()
		const manager = cells[2].textContent.trim().replace(/"/g, "")
		tldInfo.push({ tld, type, manager })
	}

	// write to file
	const rustCode = `
use phf::{phf_map, Map};
use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize)]
pub(crate) struct TldInfo {
    name: &'static str,
    category: &'static str,
    manager: &'static str,
}

pub(crate) static TLD_INFO: Map<&str, TldInfo> = phf_map!(
${tldInfo.map(({ tld, type, manager }) => `    "${tld}" => TldInfo { name: "${tld}", category: "${type}", manager: "${manager}" },`).join("\n")}
);
`

	fs.writeFileSync("../core/src/constants/tld_info.rs", rustCode)
}

writeTldInfo()
