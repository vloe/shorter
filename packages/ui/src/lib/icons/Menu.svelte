<script lang="ts">
	import { cx } from "@sh/utils/src/cva.config"

	type $Props = {
		class?: string
	}

	let { class: className }: $Props = $props()
</script>

<svg
	class={cx("size-6", className)}
	fill="currentColor"
	viewBox="0 0 24 12"
	xmlns="http://www.w3.org/2000/svg"
>
	<rect height="2" width="24"></rect>
	<rect height="2" width="24" y="5"></rect>
	<rect height="2" width="24" y="10"></rect>
</svg>
