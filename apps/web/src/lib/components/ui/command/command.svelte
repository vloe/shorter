<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { Command as CommandPrimitive } from "cmdk-sv"

	type $$Props = CommandPrimitive.CommandProps

	export let value: $$Props["value"] = undefined

	let className: null | string | undefined = undefined
	export { className as class }
</script>

<CommandPrimitive.Root
	bind:value
	class={cx(
		"flex h-full w-full flex-col overflow-hidden rounded-md bg-popover text-popover-foreground",
		className,
	)}
	{...$$restProps}
>
	<slot />
</CommandPrimitive.Root>
