import { sveltekit } from "@sveltejs/kit/vite"
import { defineConfig } from "vite"

export default defineConfig({
	plugins: [sveltekit()],
	optimizeDeps: {
		exclude: ["../webgl/pkg/sh_webgl"],
	},
	server: {
		fs: {
			allow: [".."],
		},
	},
})
