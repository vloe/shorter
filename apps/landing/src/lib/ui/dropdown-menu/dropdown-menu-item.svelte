<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "$lib/utils/cva.config"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $Props = {
		children: Snippet
	} & DropdownMenuPrimitive.ItemProps

	let { children, class: className, ...props }: $Props = $props()
</script>

<DropdownMenuPrimitive.Item
	class={cx(
		"relative flex cursor-default select-none items-center rounded-md px-2 py-1.5 text-sm text-white/85 outline-none data-[highlighted]:bg-white/10 data-[highlighted]:text-white",
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
