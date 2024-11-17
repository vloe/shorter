<script lang="ts">
	import type { WithElementRef } from "bits-ui"
	import type { HTMLInputAttributes } from "svelte/elements"

	import { Search } from "$lib/components/icons/search"
	import { cx } from "$lib/utils/cva.config"

	let {
		class: className,
		ref = $bindable(null),
		value = $bindable(),
		...restProps
	}: WithElementRef<HTMLInputAttributes> = $props()
</script>

<div class={cx("flex h-11 w-full items-center rounded-full border bg-black", className)}>
	<Search class="ml-4" />
	<input
		autofocus
		bind:this={ref}
		bind:value
		class={cx(
			"bg-black px-2 py-2 placeholder:text-white/60 focus-visible:outline-none",
			className,
		)}
		placeholder="Type any domain..."
		{...restProps}
	/>
</div>
