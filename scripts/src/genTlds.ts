import { DOMParser } from "xmldom"
import fs from "fs"
import type { Tld } from "@sh/types/src/bindings"
;(async () => {
	let tlds: Map<string, Tld> = new Map()

	// get tld names, categories, and managers
	let res = await fetch("https://www.iana.org/domains/root/db")
	let text = await res.text()
	let doc = new DOMParser().parseFromString(text, "text/html")

	let table = doc.getElementsByTagName("table")[0]
	let rows = table.getElementsByTagName("tr")

	for (let i = 1; i < rows.length; i++) {
		let cells = rows[i].getElementsByTagName("td")

		let name = cells[0].textContent?.trim()
		let category = cells[1].textContent?.trim()
		let manager = cells[2].textContent?.trim().replace(/"/g, "")

		if (name && category && manager) {
			tlds.set(name, { name, category, manager, server: "NONE" })
		}
	}

	// get whois servers
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

		let [name, server] = line.split(/\s+/, 2)
		if (tlds.has(name)) {
			let tld = tlds.get(name)
			if (tld && server && !server.startsWith("WEB") && !server.startsWith("NONE")) {
				tld.server = server.trim()
				tlds.set(name, tld)
			}
		}
	}

	res = await fetch("https://raw.githubusercontent.com/rfc1036/whois/next/new_gtlds_list")
	text = await res.text()

	for (let line of text.split("\n")) {
		if (line.length === 0 || line.startsWith("#")) continue

		let name = "." + line.trim()
		if (tlds.has(name)) {
			let tld = tlds.get(name)
			if (tld && tld.server === "NONE") {
				tld.server = `whois.nic${name}`
				tlds.set(name, tld)
			}
		}
	}

	// write to file
	const filePath = "../core/src/constants/tlds.rs"
	const linesToKeep = 15

	let existingContent = fs.readFileSync(filePath, "utf8")
	let lines = existingContent.split("\n")
	let preservedLines = lines.slice(0, linesToKeep)
	let newContent = Array.from(tlds.values())
		.map(
			({ name, category, manager, server }) =>
				`    "${name}" => Tld { name: "${name}", category: "${category}", manager: "${manager}", server: "${server}" },`,
		)
		.join("\n")
	let updatedContent = preservedLines.join("\n") + "\n" + newContent + "\n" + ");"
	fs.writeFileSync(filePath, updatedContent)
})()
