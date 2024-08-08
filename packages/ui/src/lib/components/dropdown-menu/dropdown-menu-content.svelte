<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "@sh/utils/src/cva.config"
	import { flyAndScale } from "@sh/utils/src/flyAndScale"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $Props = {
		children: Snippet
	} & DropdownMenuPrimitive.ContentProps

	let {
		children,
		class: className,
		sideOffset = 4,
		transition = flyAndScale,
		transitionConfig,
		...props
	}: $Props = $props()
</script>

<DropdownMenuPrimitive.Content
	class={cx(
		"z-50 min-w-[8rem] rounded-md border bg-popover p-1 text-popover-foreground shadow-md focus:outline-none",
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
