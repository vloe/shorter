<script lang="ts">
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"
	import type { Snippet } from "svelte"
	import { cx } from "$lib/utils/cva.config"

	type $Props = DropdownMenuPrimitive.ItemProps & {
		children: Snippet
	}

	let { children, class: className, ...props }: $Props = $props()
</script>

<DropdownMenuPrimitive.Item
	class={cx(
		"relative flex cursor-default select-none items-center rounded-md px-2 py-1.5 text-sm text-white/85 outline-none data-[highlighted]:bg-white/10 data-[highlighted]:text-white",
		className,
	)}
	on:click
	on:keydown
	on:focusin
	on:focusout
	on:pointerdown
	on:pointerleave
	on:pointermove
	{...props}
>
	{@render children()}
</DropdownMenuPrimitive.Item>
