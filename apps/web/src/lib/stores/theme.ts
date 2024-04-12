import { writable } from "svelte/store"
import { browser } from "$app/environment"

type Theme = "light" | "dark"

const getThemePreference = (): Theme => {
	if (browser) {
		if (localStorage.getItem("theme") === "dark") return "dark"
		if (localStorage.getItem("theme") === "light") return "light"
		if (window.matchMedia("(prefers-color-scheme: dark)").matches) return "dark"
	}
	return "light"
}

export const theme = writable<Theme>(getThemePreference())

theme.subscribe((value) => {
	if (browser) {
		const isDark = value === "dark"
		document.documentElement.classList[isDark ? "add" : "remove"]("dark")
		localStorage.setItem("theme", isDark ? "dark" : "light")
	}
})
