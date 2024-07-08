export const scrollToElement = (id: string) => {
	let element = document.getElementById(id)
	if (element) {
		element.scrollIntoView({ behavior: "smooth" })
	}
}
