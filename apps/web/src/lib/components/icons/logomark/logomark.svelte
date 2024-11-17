<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	type $Props = {
		class?: string
	}

	let { class: className }: $Props = $props()
</script>

<svg
	class={cx("size-7", className)}
	fill="currentColor"
	viewBox="0 0 600 600"
	xmlns="http://www.w3.org/2000/svg"
>
	<path d="M299.75,600c110.34,0,199.79-89.45,199.79-199.79s-89.45-199.79-199.79-199.79v399.59Z" />
	<path d="M300.25,399.59c-110.34,0-199.79-89.45-199.79-199.79S189.9,0,300.25,0v399.59Z" />
</svg>
