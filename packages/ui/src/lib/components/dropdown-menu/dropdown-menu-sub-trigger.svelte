<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "@sh/utils/src/cva.config"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"
	import ChevronRight from "svelte-radix/ChevronRight.svelte"

	type $Props = {
		children: Snippet
		inset?: boolean
	} & DropdownMenuPrimitive.SubTriggerProps

	let { children, class: className, inset, ...props }: $Props = $props()
</script>

<DropdownMenuPrimitive.SubTrigger
	class={cx(
		"flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none data-[highlighted]:bg-accent data-[state=open]:bg-accent data-[highlighted]:text-accent-foreground data-[state=open]:text-accent-foreground",
		inset && "pl-8",
		className,
	)}
	{...props}
	on:click
	on:focusin
	on:focusout
	on:keydown
	on:pointerleave
	on:pointermove
>
	{@render children()}
	<ChevronRight class="ml-auto h-4 w-4" />
</DropdownMenuPrimitive.SubTrigger>
