<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { flyAndScale } from "$lib/utils/flyAndScale"
	import { Popover as PopoverPrimitive } from "bits-ui"
	import type { Snippet } from "svelte"

	type $Props = PopoverPrimitive.ContentProps & {
		children: Snippet
	}

	let {
		children,
		class: className,
		transition = flyAndScale,
		transitionConfig,
		align = "center",
		sideOffset = 8,
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
