export const scrollTo = (id: string) => {
	let elmt = document.getElementById(id)
	if (elmt) {
		elmt.scrollIntoView({ behavior: "smooth" })
	}
}
