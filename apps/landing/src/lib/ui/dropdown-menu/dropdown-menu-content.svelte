<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"
	import type { Snippet } from "svelte"
	import { cx } from "$lib/utils/cva.config"
	import { flyAndScale } from "$lib/utils/flyAndScale"

	type $Props = DropdownMenuPrimitive.ContentProps & {
		children: Snippet
	}

	let {
		children,
		sideOffset = 4,
		transition = flyAndScale,
		class: className,
		...props
	}: $Props = $props()
</script>

<DropdownMenuPrimitive.Content
	{transition}
	{sideOffset}
	class={cx(
		"z-50 min-w-[8rem] rounded-lg border border-white/15 bg-black p-1 text-white shadow-md focus:outline-none",
		className,
	)}
	{...props}
	on:keydown
>
	{@render children()}
</DropdownMenuPrimitive.Content>
