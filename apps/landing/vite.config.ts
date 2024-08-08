import { mergeConfig } from "vite"

import baseConfig from "../../packages/config/vite/base"

export default mergeConfig(baseConfig, {
	server: {
		port: 3000,
	},
})
