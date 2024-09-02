<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	type $Props = {
		class?: string
	}

	let { class: className }: $Props = $props()
</script>

<svg
	class={cx("size-3.5", className)}
	fill="currentColor"
	viewBox="0 0 512 512"
	xmlns="http://www.w3.org/2000/svg"
>
	<path
		d="M236.8 288a51.2 51.2 0 1 0 0-102.4 51.2 51.2 0 1 0 0 102.4zM434.9 398.7c-10.9 13.1-23 25.3-36.2 36.2l68.5 68.5 36.2-36.2-68.5-68.5zM236.8 32a204.8 204.8 0 1 1 0 409.6 204.8 204.8 0 1 1 0-409.6zm0 51.2a153.6 153.6 0 1 0 0 307.2 153.6 153.6 0 1 0 0-307.2z"
	/>
</svg>
