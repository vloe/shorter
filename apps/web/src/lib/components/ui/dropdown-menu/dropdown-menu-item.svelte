<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	let {
		class: className,
		inset,
		ref = $bindable(null),
		...restProps
	}: {
		inset?: boolean
	} & DropdownMenuPrimitive.ItemProps = $props()
</script>

<DropdownMenuPrimitive.Item
	bind:ref
	class={cx(
		"relative flex cursor-default select-none items-center gap-2 rounded-md px-2 py-1.5 text-sm outline-none transition-colors data-[disabled]:pointer-events-none data-[highlighted]:bg-white/10 data-[disabled]:opacity-50",
		inset && "pl-8",
		className,
	)}
	{...restProps}
/>
