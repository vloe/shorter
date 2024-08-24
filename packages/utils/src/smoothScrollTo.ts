export const smoothScrollTo = (id: string) => {
	const elmt = document.getElementById(id)

	if (!elmt) return

	elmt.scrollIntoView({
		behavior: "smooth",
		block: "center",
	})
}
