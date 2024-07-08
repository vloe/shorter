import { DOMParser } from "xmldom"
import fs from "fs"
;(async () => {
	let tldInfo = []

	// get tlds, categories, and managers
	let res = await fetch("https://www.iana.org/domains/root/db")
	let text = await res.text()
	let doc = new DOMParser().parseFromString(text, "text/html")

	let table = doc.getElementsByTagName("table")[0]
	let rows = table.getElementsByTagName("tr")

	for (let i = 1; i < rows.length; i++) {
		let cells = rows[i].getElementsByTagName("td")
		let tld = cells[0].textContent?.trim()
		let type = cells[1].textContent?.trim()
		let manager = cells[2].textContent?.trim().replace(/"/g, "")
		tldInfo.push({ tld, type, manager })
	}

	// write to file
	const filePath = "../core/src/constants/tld_info.rs"
	const linesToKeep = 14

	let existingContent = fs.readFileSync(filePath, "utf8")
	let lines = existingContent.split("\n")
	let preservedLines = lines.slice(0, linesToKeep)
	let newContent = `${tldInfo.map(({ tld, type, manager }) => `    "${tld}" => TldInfo { name: "${tld}", category: "${type}", manager: "${manager}" },`).join("\n")}`
	let updatedContent = preservedLines.join("\n") + "\n" + newContent + "\n" + ");"
	fs.writeFileSync(filePath, updatedContent)
})()
