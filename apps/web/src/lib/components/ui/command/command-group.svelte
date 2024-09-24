<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { Command as CommandPrimitive } from "cmdk-sv"
	type $$Props = CommandPrimitive.GroupProps

	let className: null | string | undefined = undefined
	export { className as class }
</script>

<CommandPrimitive.Group
	class={cx(
		"overflow-hidden p-1 text-foreground [&_[data-cmdk-group-heading]]:px-2 [&_[data-cmdk-group-heading]]:py-1.5 [&_[data-cmdk-group-heading]]:text-xs [&_[data-cmdk-group-heading]]:font-medium [&_[data-cmdk-group-heading]]:text-muted-foreground",
		className,
	)}
	{...$$restProps}
>
	<slot />
</CommandPrimitive.Group>
