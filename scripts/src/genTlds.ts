import { DOMParser } from "xmldom"
import fs from "fs"
;(async () => {
	let tlds = new Map()

	// get tlds, categories, and managers
	let res = await fetch("https://www.iana.org/domains/root/db")
	let text = await res.text()
	let doc = new DOMParser().parseFromString(text, "text/html")

	let table = doc.getElementsByTagName("table")[0]
	let rows = table.getElementsByTagName("tr")

	for (let i = 1; i < rows.length; i++) {
		let cells = rows[i].getElementsByTagName("td")
		let tld = cells[0].textContent?.trim()
		let category = cells[1].textContent?.trim()
		let manager = cells[2].textContent?.trim().replace(/"/g, "")
		if (tld) {
			tlds.set(tld, { tld, category, manager })
		}
	}

	// get the tlds that has a whois server
	let tldsWithServer = new Map()
	res = await fetch("https://raw.githubusercontent.com/rfc1036/whois/next/tld_serv_list")
	text = await res.text()

	for (let line of text.split("\n")) {
		if (line.length === 0 || line.startsWith("#")) continue
		line = line
			.replace("NONE", "")
			.replace("VERISIGN", "")
			.replace("RECURSIVE", "")
			.replace("ARPA", "")
			.replace("IP6", "")
			.split("#")[0]

		let [tld, server] = line.split(/\s+/, 2)
		if (server && !server.startsWith("WEB") && !server.startsWith("NONE")) {
			tldsWithServer.set(tld, tld)
		}
	}

	res = await fetch("https://raw.githubusercontent.com/rfc1036/whois/next/new_gtlds_list")
	text = await res.text()

	for (let line of text.split("\n")) {
		if (line.length === 0 || line.startsWith("#")) continue

		let tld = "." + line.trim()
		if (tld && !tldsWithServer.has(tld)) {
			tldsWithServer.set(tld, tld)
		}
	}

	// remove tlds that don't have a server
	for (let tld of tlds.keys()) {
		if (!tldsWithServer.has(tld)) {
			tlds.delete(tld)
		}
	}

	// write to file
	const filePath = "../core/src/constants/tlds.rs"
	const linesToKeep = 14

	let existingContent = fs.readFileSync(filePath, "utf8")
	let lines = existingContent.split("\n")
	let preservedLines = lines.slice(0, linesToKeep)
	let newContent = Array.from(tlds.values())
		.map(
			({ tld, category, manager }) =>
				`    "${tld}" => Tld { name: "${tld}", category: "${category}", manager: "${manager}" },`,
		)
		.join("\n")
	let updatedContent = preservedLines.join("\n") + "\n" + newContent + "\n" + ");"
	fs.writeFileSync(filePath, updatedContent)
})()
