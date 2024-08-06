<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	type $Props = {
		class?: string
	}

	let { class: className }: $Props = $props()
</script>

<svg viewBox="0 0 24 12" fill="currentColor" class={cx("size-6", className)}>
	<rect width="24" height="2" />
	<rect y="5" width="24" height="2" />
	<rect y="10" width="24" height="2" />
</svg>
