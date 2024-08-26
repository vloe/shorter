<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "@sh/utils/src/cva.config"
	import { Command as CommandPrimitive } from "cmdk-sv"

	type $Props = {
		children: Snippet
	} & CommandPrimitive.ListProps

	let { children, class: className, ...props }: $Props = $props()
</script>

<CommandPrimitive.List
	class={cx("max-h-[300px] overflow-y-auto overflow-x-hidden", className)}
	{...props}
>
	{@render children()}
</CommandPrimitive.List>
