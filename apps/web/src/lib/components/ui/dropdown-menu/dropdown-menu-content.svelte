<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { flyAndScale } from "$lib/utils/flyAndScale"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $$Props = DropdownMenuPrimitive.ContentProps

	let className: $$Props["class"] = undefined
	export let sideOffset: $$Props["sideOffset"] = 4
	export let transition: $$Props["transition"] = flyAndScale
	export let transitionConfig: $$Props["transitionConfig"] = undefined
	export { className as class }
</script>

<DropdownMenuPrimitive.Content
	class={cx(
		"z-50 min-w-[8rem] rounded-md border bg-popover p-1 text-popover-foreground shadow-md focus:outline-none",
		className,
	)}
	{sideOffset}
	{transition}
	{transitionConfig}
	{...$$restProps}
	on:keydown
>
	<slot />
</DropdownMenuPrimitive.Content>
