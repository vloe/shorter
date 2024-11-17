<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	type $Props = {
		class?: string
	}

	let { class: className }: $Props = $props()
</script>

<svg
	class={cx("size-2.5 stroke-2", className)}
	fill="none"
	stroke="currentColor"
	viewBox="0 0 10 10"
	xmlns="http://www.w3.org/2000/svg"
>
	<path class="opacity-0 transition-opacity group-hover:opacity-100" d="M0 5h7" />
	<path class="transition-all ease-in-out group-hover:translate-x-0.5" d="M1 1l4 4-4 4" />
</svg>
