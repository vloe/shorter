import { fontFamily } from "tailwindcss/defaultTheme"

export default {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	plugins: [],
	theme: {
		fontFamily: {
			sans: ["Manrope", ...fontFamily.sans],
		},
	},
}
