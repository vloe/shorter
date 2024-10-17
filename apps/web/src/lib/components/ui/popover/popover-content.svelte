<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "$lib/utils/cva.config"
	import { flyAndScale } from "$lib/utils/flyAndScale"
	import { Popover as PopoverPrimitive } from "bits-ui"

	type $Props = {
		children: Snippet
	} & PopoverPrimitive.ContentProps

	let {
		align = "center",
		children,
		class: className,
		sideOffset = 8,
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
	class={cx("z-50 w-72 rounded-lg border bg-black p-3 shadow-md outline-none", className)}
>
	{@render children()}
</PopoverPrimitive.Content>
