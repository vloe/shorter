<script lang="ts">
<<<<<<< HEAD
	import type { Snippet } from "svelte"

	import { cx } from "$lib/utils/cva.config"
	import { flyAndScale } from "$lib/utils/flyAndScale"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $Props = {
		children: Snippet
	} & DropdownMenuPrimitive.ContentProps

	let {
		children,
		class: className,
		sideOffset = 2,
		transition = flyAndScale,
=======
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"
	import type { Snippet } from "svelte"
	import { cx } from "$lib/utils/cva.config"
	import { flyAndScale } from "$lib/utils/flyAndScale"

	type $Props = DropdownMenuPrimitive.ContentProps & {
		children: Snippet
	}

	let {
		children,
		sideOffset = 2,
		transition = flyAndScale,
		class: className,
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
		...props
	}: $Props = $props()
</script>

<DropdownMenuPrimitive.Content
<<<<<<< HEAD
=======
	{transition}
	{sideOffset}
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
	class={cx(
		"z-50 min-w-[8rem] rounded-lg border border-white/30 bg-black p-1 text-white shadow-md focus:outline-none",
		className,
	)}
<<<<<<< HEAD
	{sideOffset}
	{transition}
=======
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
	{...props}
	on:keydown
>
	{@render children()}
</DropdownMenuPrimitive.Content>
