<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { Popover as PopoverPrimitive } from "bits-ui"

	let {
		align = "center",
		class: className,
		ref = $bindable(null),
		sideOffset = 4,
		...restProps
	}: PopoverPrimitive.ContentProps = $props()
</script>

<PopoverPrimitive.Portal>
	<PopoverPrimitive.Content
		{align}
		bind:ref
		class={cx(
			"z-50 w-72 rounded-lg border bg-black p-4 shadow-md outline-none data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2",
			className,
		)}
		{sideOffset}
		{...restProps}
	/>
</PopoverPrimitive.Portal>
