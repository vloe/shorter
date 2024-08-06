export default {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	theme: {
		fontFamily: {
			sans: ["Manrope", "ui-sans-serif", "sans-serif"],
			aeonik: ["Aeonik", "ui-sans-serif", "sans-serif"],
		},
		extend: {
			colors: {
				shpurple: "#B78EFF",
				shblue: "#3C72EF",
			},
			screens: {
				xs: "480px",
			},
		},
	},
	plugins: [],
}
