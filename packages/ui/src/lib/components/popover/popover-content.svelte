<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "@sh/utils/src/cva.config"
	import { flyAndScale } from "@sh/utils/src/flyAndScale"
	import { Popover as PopoverPrimitive } from "bits-ui"

	type $Props = {
		children: Snippet
	} & PopoverPrimitive.ContentProps

	let {
		align = "center",
		children,
		class: className,
		sideOffset = 4,
		transition = flyAndScale,
		transitionConfig,
		...props
	}: $Props = $props()
</script>

<PopoverPrimitive.Content
	{align}
	{sideOffset}
	{transition}
	{transitionConfig}
	{...props}
	class={cx(
		"z-50 w-72 rounded-md border bg-popover p-4 text-popover-foreground shadow-md outline-none",
		className,
	)}
>
	{@render children()}
</PopoverPrimitive.Content>
