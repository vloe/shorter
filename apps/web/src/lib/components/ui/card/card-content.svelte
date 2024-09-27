<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	type $$Props = HTMLAttributes<HTMLDivElement>

	let className: $$Props["class"] = undefined
	export { className as class }
</script>

<div class={cx("p-6", className)} {...$$restProps}>
	<slot />
</div>
