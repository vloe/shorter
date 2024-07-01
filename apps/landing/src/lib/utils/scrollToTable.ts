export const scrollToTable = () => {
	const table = document.getElementById("shorten-table")
	if (table) {
		table.scrollIntoView({ behavior: "smooth" })
	}
}
