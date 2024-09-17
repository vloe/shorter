<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	type $Props = {
		class?: string
	}

	let { class: className }: $Props = $props()
</script>

<svg
	class={cx("size-4", className)}
	fill="currentColor"
	viewBox="0 0 20 20"
	xmlns="http://www.w3.org/2000/svg"
>
	<path
		clip-rule="evenodd"
		d="M2 4.25A2.25 2.25 0 014.25 2h11.5A2.25 2.25 0 0118 4.25v1.5A2.25 2.25 0 0115.75 8H4.25A2.25 2.25 0 012 5.75v-1.5zm0 10A2.25 2.25 0 014.25 12h11.5A2.25 2.25 0 0118 14.25v1.5A2.25 2.25 0 0115.75 18H4.25A2.25 2.25 0 012 15.75v-1.5z"
		fill-rule="evenodd"
	/>
</svg>
