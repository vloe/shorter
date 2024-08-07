<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	type $Props = {
		class?: string
	}

	let { class: className }: $Props = $props()
</script>

<<<<<<< HEAD
<svg class={cx("size-6", className)} fill="currentColor" viewBox="0 0 24 12">
	<rect height="2" width="24" />
	<rect height="2" width="24" y="5" />
	<rect height="2" width="24" y="10" />
=======
<svg viewBox="0 0 24 12" fill="currentColor" class={cx("size-6", className)}>
	<rect width="24" height="2" />
	<rect y="5" width="24" height="2" />
	<rect y="10" width="24" height="2" />
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
</svg>
