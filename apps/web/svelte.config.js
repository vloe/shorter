import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

const config = {
	kit: {
		adapter: adapter(),
		paths: {
			relative: false,
		},
	},
	preprocess: vitePreprocess()
};

export default config;
