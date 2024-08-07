export default {
	content: ["./src/**/*.{html,js,svelte,ts}"],
	plugins: [],
	theme: {
		extend: {
			colors: {
				shblue: "#3C72EF",
				shpurple: "#B78EFF",
			},
			screens: {
				xs: "480px",
			},
		},
		fontFamily: {
			aeonik: ["Aeonik", "ui-sans-serif", "sans-serif"],
			sans: ["Manrope", "ui-sans-serif", "sans-serif"],
		},
	},
}
