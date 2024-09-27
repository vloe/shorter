<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { flyAndScale } from "$lib/utils/flyAndScale"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $$Props = DropdownMenuPrimitive.SubContentProps

	let className: $$Props["class"] = undefined
	export let transition: $$Props["transition"] = flyAndScale
	export let transitionConfig: $$Props["transitionConfig"] = {
		x: -10,
		y: 0,
	}
	export { className as class }
</script>

<DropdownMenuPrimitive.SubContent
	class={cx(
		"z-50 min-w-[8rem] rounded-md border bg-popover p-1 text-popover-foreground shadow-lg focus:outline-none",
		className,
	)}
	{transition}
	{transitionConfig}
	{...$$restProps}
	on:focusout
	on:keydown
	on:pointermove
>
	<slot />
</DropdownMenuPrimitive.SubContent>
