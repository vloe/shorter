import puppeteer from "puppeteer"
import { writeFile } from "fs/promises"

const IANA_TLDS_URL = "https://www.iana.org/domains/root/db"
const TLDS_FILE = "../apps/server/src/constants/tlds.rs"

;(async () => {
	const browser = await puppeteer.launch({ headless: false })
	const page = await browser.newPage()
	await page.setViewport({ width: 1080, height: 1024 })

	const tlds = await getIanaTlds(page)
	writeTlds(tlds)
	console.log("done")
})()

async function getIanaTlds(page) {
	await page.goto(IANA_TLDS_URL)

	const tlds = await page.evaluate(() => {
		const rows = Array.from(document.querySelectorAll("table#tld-table tbody tr"))
		return rows
			.map((row) => {
				const cells = Array.from(row.querySelectorAll("td"))
				if (cells.length < 3) return null

				const name = cells[0].textContent.trim().toLowerCase().replace(".", "")
				const category = cells[1].textContent.trim().toLowerCase()
				const manager = cells[2].textContent
					.trim()
					.toLowerCase()
					.replace("not assigned", "")
					.replace(/"/g, "")

				if (!name || !category || !manager) return null

				return {
					name,
					category,
					manager,
					buyable: false,
				}
			})
			.filter((tld) => tld !== null)
	})

	if (tlds.length === 0) {
		throw new Error("failed to get iana tlds, mby the site is down or changed?")
	}
	return tlds
}

async function writeTlds(tlds) {
	let content = "use phf::{phf_map, Map};\n"
	content += "use serde::Serialize;\n"
	content += "use std::clone::Clone;\n"
	content += "use typeshare::typeshare;\n\n"

	content += "#[typeshare]\n"
	content += "#[derive(Serialize, Clone)]\n"
	content += "pub struct Tld {\n"
	content += "    pub name: &'static str,\n"
	content += "    pub category: &'static str,\n"
	content += "    pub manager: &'static str,\n"
	content += "    pub buyable: bool,\n"
	content += "}\n\n"

	content += "pub static TLDS: Map<&str, Tld> = phf_map!(\n"

	for (const tld of tlds) {
		content += `    "${tld.name}" => Tld { name: "${tld.name}", category: "${tld.category}", manager: "${tld.manager}", buyable: ${tld.buyable} },\n`
	}

	content += ");\n"

	await writeFile(TLDS_FILE, content)
}
