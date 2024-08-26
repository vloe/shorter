<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "@sh/utils/src/cva.config"
	import { Command as CommandPrimitive } from "cmdk-sv"
	type $Props = {
		children: Snippet
	} & CommandPrimitive.GroupProps

	let { children, class: className, ...props }: $Props = $props()
</script>

<CommandPrimitive.Group
	class={cx(
		"overflow-hidden p-1 text-foreground [&_[data-cmdk-group-heading]]:px-2 [&_[data-cmdk-group-heading]]:py-1.5 [&_[data-cmdk-group-heading]]:text-xs [&_[data-cmdk-group-heading]]:font-medium [&_[data-cmdk-group-heading]]:text-muted-foreground",
		className,
	)}
	{...props}
>
	{@render children()}
</CommandPrimitive.Group>
