<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "$lib/utils/cva.config"
	import { flyAndScale } from "$lib/utils/flyAndScale"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $Props = {
		children: Snippet
	} & DropdownMenuPrimitive.ContentProps

	let {
		children,
		class: className,
		sideOffset = 2,
		transition = flyAndScale,
		...props
	}: $Props = $props()
</script>

<DropdownMenuPrimitive.Content
	class={cx(
		"z-50 min-w-[8rem] rounded-lg border border-white/30 bg-black p-1 text-white shadow-md focus:outline-none",
		className,
	)}
	{sideOffset}
	{transition}
	{...props}
	on:keydown
>
	{@render children()}
</DropdownMenuPrimitive.Content>
