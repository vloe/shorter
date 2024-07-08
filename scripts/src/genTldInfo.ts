import { DOMParser } from "xmldom"
import fs from "fs"
;(async () => {
	let tldInfo = new Map()

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
			tldInfo.set(tld, { tld, category, manager, server: null })
		}
	}

	// get tld servers
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
		if (tldInfo.has(tld)) {
			let info = tldInfo.get(tld)
			if (server && !server.startsWith("WEB") && !server.startsWith("NONE")) {
				info.server = server.trim()
			}
			tldInfo.set(tld, info)
		}
	}

	res = await fetch("https://raw.githubusercontent.com/rfc1036/whois/next/new_gtlds_list")
	text = await res.text()

	for (let line of text.split("\n")) {
		if (line.length === 0 || line.startsWith("#")) continue

		let tld = "." + line.trim()
		if (tldInfo.has(tld)) {
			let info = tldInfo.get(tld)
			if (tld && info.server === null) {
				info.server = `whois.nic${tld}`
			}
			tldInfo.set(tld, info)
		}
	}

	// remove tlds that don't have a server
	for (let tld of tldInfo.keys()) {
		if (tldInfo.get(tld).server === null) {
			tldInfo.delete(tld)
		}
	}

	// write to file
	const filePath = "../core/src/constants/tld_info.rs"
	const linesToKeep = 15

	let existingContent = fs.readFileSync(filePath, "utf8")
	let lines = existingContent.split("\n")
	let preservedLines = lines.slice(0, linesToKeep)
	let newContent = Array.from(tldInfo.values())
		.map(
			({ tld, category, manager, server }) =>
				`    "${tld}" => TldInfo { name: "${tld}", category: "${category}", manager: "${manager}", server: "${server}" },`,
		)
		.join("\n")
	let updatedContent = preservedLines.join("\n") + "\n" + newContent + "\n" + ");"
	fs.writeFileSync(filePath, updatedContent)
})()
