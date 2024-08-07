import { sveltekit } from "@sveltejs/kit/vite"
import { defineConfig } from "vite"

export default defineConfig({
	optimizeDeps: {
		exclude: ["../webgl/pkg/sh_webgl"],
	},
	plugins: [sveltekit()],
	server: {
		fs: {
			allow: [".."],
		},
		port: 3000,
	},
})
