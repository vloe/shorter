<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	type $$Props = HTMLAttributes<HTMLDivElement>

	let className: $$Props["class"] = undefined
	export { className as class }
</script>

<div class={cx("animate-pulse rounded-md bg-primary/10", className)} {...$$restProps}></div>
