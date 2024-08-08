import { mergeConfig } from "vite"

import baseConfig from "../config/vite/base"

export default mergeConfig(baseConfig, {
    server: {
        port: 3002
    }
})
