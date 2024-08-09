import { mergeConfig } from "vite"

import baseConfig from "../../packages/config/vite/base"

export default mergeConfig(baseConfig, {
	optimizeDeps: {
		exclude: ["../webgl/pkg/sh_webgl"],
	},
	server: {
		fs: {
			allow: [".."],
		},
		port: 3000,
	},
})
