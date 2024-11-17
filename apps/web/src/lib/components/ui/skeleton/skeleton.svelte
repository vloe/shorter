<script lang="ts">
	import type { WithElementRef, WithoutChildren } from "bits-ui"
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	let {
		class: className,
		ref = $bindable(null),
		...restProps
	}: WithoutChildren<WithElementRef<HTMLAttributes<HTMLDivElement>>> = $props()
</script>

<div
	bind:this={ref}
	class={cx("animate-pulse rounded-lg bg-white/10", className)}
	{...restProps}
></div>
