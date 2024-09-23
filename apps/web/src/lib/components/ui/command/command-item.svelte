<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "$lib/utils/cva.config"
	import { Command as CommandPrimitive } from "cmdk-sv"

	type $Props = {
		children: Snippet
	} & CommandPrimitive.CommandProps

	let { children, class: className, ...props }: $Props = $props()
</script>

<CommandPrimitive.Item
	class={cx(
		"relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none aria-selected:bg-white/15 data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
		className,
	)}
	{...props}
>
	{@render children()}
</CommandPrimitive.Item>
