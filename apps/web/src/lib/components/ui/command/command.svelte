<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "$lib/utils/cva.config"
	import { Command as CommandPrimitive } from "cmdk-sv"

	type $Props = {
		children: Snippet
	} & CommandPrimitive.CommandProps

	let { children, class: className, value = $bindable(), ...props }: $Props = $props()
</script>

<CommandPrimitive.Root
	bind:value
	class={cx("flex h-full w-full flex-col overflow-hidden rounded-md bg-black", className)}
	{...props}
>
	{@render children()}
</CommandPrimitive.Root>
