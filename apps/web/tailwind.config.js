import { fontFamily } from "tailwindcss/defaultTheme"

const config = {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		extend: {
			fontFamily: {
				sans: ["Manrope", ...fontFamily.sans],
			},
		},
	},
}

export default config
