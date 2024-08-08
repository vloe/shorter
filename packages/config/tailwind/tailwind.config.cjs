import { fontFamily } from "tailwindcss/defaultTheme";

/** @type {import('tailwindcss').Config} */
module.exports = {
	content: [
        "./src/**/*.{html,js,svelte,ts}",
        "../../packages/ui/src/**/*.{html,js,svelte,ts}"
    ],
	darkMode: ["class"],
	safelist: ["dark"],
	theme: {
		extend: {
			colors: {
				accent: {
					DEFAULT: "hsl(var(--accent) / <alpha-value>)",
					foreground: "hsl(var(--accent-foreground) / <alpha-value>)"
				},
				background: "hsl(var(--background) / <alpha-value>)",
				border: "hsl(var(--border) / <alpha-value>)",
				card: {
					DEFAULT: "hsl(var(--card) / <alpha-value>)",
					foreground: "hsl(var(--card-foreground) / <alpha-value>)"
				},
				destructive: {
					DEFAULT: "hsl(var(--destructive) / <alpha-value>)",
					foreground: "hsl(var(--destructive-foreground) / <alpha-value>)"
				},
				foreground: "hsl(var(--foreground) / <alpha-value>)",
				input: "hsl(var(--input) / <alpha-value>)",
				muted: {
					DEFAULT: "hsl(var(--muted) / <alpha-value>)",
					foreground: "hsl(var(--muted-foreground) / <alpha-value>)"
				},
				popover: {
					DEFAULT: "hsl(var(--popover) / <alpha-value>)",
					foreground: "hsl(var(--popover-foreground) / <alpha-value>)"
				},
				primary: {
					DEFAULT: "hsl(var(--primary) / <alpha-value>)",
					foreground: "hsl(var(--primary-foreground) / <alpha-value>)"
				},
				ring: "hsl(var(--ring) / <alpha-value>)",
				secondary: {
					DEFAULT: "hsl(var(--secondary) / <alpha-value>)",
					foreground: "hsl(var(--secondary-foreground) / <alpha-value>)"
				}
			},
			fontFamily: {
				sans: [...fontFamily.sans]
			}
		}
	},
};
