<script lang="ts">
<<<<<<< HEAD
	import type { Snippet } from "svelte"

	import { cx } from "$lib/utils/cva.config"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $Props = {
		children: Snippet
	} & DropdownMenuPrimitive.ItemProps
=======
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"
	import type { Snippet } from "svelte"
	import { cx } from "$lib/utils/cva.config"

	type $Props = DropdownMenuPrimitive.ItemProps & {
		children: Snippet
	}
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c

	let { children, class: className, ...props }: $Props = $props()
</script>

<DropdownMenuPrimitive.Item
	class={cx(
		"relative flex cursor-default select-none items-center rounded-md px-2 py-1.5 text-sm text-white/85 outline-none data-[highlighted]:bg-white/10 data-[highlighted]:text-white",
		className,
	)}
	on:click
<<<<<<< HEAD
	on:focusin
	on:focusout
	on:keydown
=======
	on:keydown
	on:focusin
	on:focusout
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
	on:pointerdown
	on:pointerleave
	on:pointermove
	{...props}
>
	{@render children()}
</DropdownMenuPrimitive.Item>
