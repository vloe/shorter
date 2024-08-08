<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "@sh/utils/src/cva.config"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $$Props = {
		children: Snippet
		inset?: boolean
	} & DropdownMenuPrimitive.ItemProps

	let { children, class: className, inset, ...props }: $$Props = $props()
</script>

<DropdownMenuPrimitive.Item
	class={cx(
		"relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-[disabled]:pointer-events-none data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground data-[disabled]:opacity-50",
		inset && "pl-8",
		className,
	)}
	on:click
	on:focusin
	on:focusout
	on:keydown
	on:pointerdown
	on:pointerleave
	on:pointermove
	{...props}
>
	{@render children()}
</DropdownMenuPrimitive.Item>
