<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "@sh/utils/src/cva.config"
	import { Command as CommandPrimitive } from "cmdk-sv"

	type $Props = {
		children: Snippet
	} & CommandPrimitive.ItemProps

	let { asChild = false, children, class: className, ...props }: $Props = $props()
</script>

<CommandPrimitive.Item
	{asChild}
	class={cx(
		"relative flex cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none aria-selected:bg-accent aria-selected:text-accent-foreground data-[disabled]:pointer-events-none data-[disabled]:opacity-50",
		className,
	)}
	{...props}
	let:action
	let:attrs
>
	{@render children()}
</CommandPrimitive.Item>
