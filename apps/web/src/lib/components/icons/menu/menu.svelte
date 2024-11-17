<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	type $Props = {
		class?: string
	}

	let { class: className }: $Props = $props()
</script>

<svg
	class={cx("size-6", className)}
	fill="currentColor"
	viewBox="0 0 16 10"
	xmlns="http://www.w3.org/2000/svg"
>
	<rect height="2" rx="1" width="16" y="8"></rect>
	<rect height="2" rx="1" width="16" y="4"></rect>
	<rect height="2" rx="1" width="16"></rect>
</svg>
