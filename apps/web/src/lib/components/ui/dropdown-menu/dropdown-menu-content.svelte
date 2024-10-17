<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { flyAndScale } from "$lib/utils/flyAndScale"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"
	import type { Snippet } from "svelte"

	type $Props = DropdownMenuPrimitive.ContentProps & {
		children: Snippet
	}

	let {
		children,
		sideOffset = 8,
		transition = flyAndScale,
		class: className,
		transitionConfig,
		...props
	}: $Props = $props()
</script>

<DropdownMenuPrimitive.Content
	class={cx(
		"z-50 min-w-[8rem] rounded-lg border bg-black p-1 shadow-md focus:outline-none",
		className,
	)}
	{sideOffset}
	{transition}
	{transitionConfig}
	{...props}
	on:keydown
>
	{@render children()}
</DropdownMenuPrimitive.Content>
