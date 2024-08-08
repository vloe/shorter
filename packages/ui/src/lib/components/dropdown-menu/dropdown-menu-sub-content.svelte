<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "@sh/utils/src/cva.config"
	import { flyAndScale } from "@sh/utils/src/flyAndScale"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $Props = {
		children: Snippet
	} & DropdownMenuPrimitive.SubContentProps

	let {
		children,
		class: className,
		transition = flyAndScale,
		transitionConfig = {
			x: -10,
			y: 0,
		},
		...props
	}: $Props = $props()
</script>

<DropdownMenuPrimitive.SubContent
	class={cx(
		"z-50 min-w-[8rem] rounded-md border bg-popover p-1 text-popover-foreground shadow-lg focus:outline-none",
		className,
	)}
	{transition}
	{transitionConfig}
	{...props}
	on:focusout
	on:keydown
	on:pointermove
>
	{@render children()}
</DropdownMenuPrimitive.SubContent>
