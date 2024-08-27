<script lang="ts">
	import { cx } from "@sh/utils/src/cva.config"

	type $Props = {
		class?: string
	}

	let { class: className }: $Props = $props()
</script>

<svg
	class={cx("size-5", className)}
	fill="currentColor"
	viewBox="0 0 24 24"
	xmlns="http://www.w3.org/2000/svg"
>
	<path
		d="M5.27 9.76A7.08 7.08 0 0 1 16.42 6.5L19.9 3A11.97 11.97 0 0 0 1.24 6.65l4.03 3.11Z"
		fill="#EA4335"
	/>
	<path
		d="M16.04 18.01A7.4 7.4 0 0 1 12 19.1a7.08 7.08 0 0 1-6.72-4.82l-4.04 3.06A11.96 11.96 0 0 0 12 24a11.4 11.4 0 0 0 7.83-3l-3.79-2.99Z"
		fill="#34A853"
	/>
	<path
		d="M19.83 21c2.2-2.05 3.62-5.1 3.62-9 0-.7-.1-1.47-.27-2.18H12v4.63h6.44a5.4 5.4 0 0 1-2.4 3.56l3.8 2.99Z"
		fill="#4A90E2"
	/>
	<path
		d="M5.28 14.27a7.12 7.12 0 0 1-.01-4.5L1.24 6.64A11.93 11.93 0 0 0 0 12c0 1.92.44 3.73 1.24 5.33l4.04-3.06Z"
		fill="#FBBC05"
	/>
</svg>
