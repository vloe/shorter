<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	type $Props = {
		class?: string
	}

	let { class: className }: $Props = $props()
</script>

<svg class={cx("size-6", className)} fill="currentColor" viewBox="0 0 24 12">
	<rect height="2" width="24" />
	<rect height="2" width="24" y="5" />
	<rect height="2" width="24" y="10" />
</svg>
