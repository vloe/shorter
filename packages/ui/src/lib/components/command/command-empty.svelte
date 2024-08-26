<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "@sh/utils/src/cva.config"
	import { Command as CommandPrimitive } from "cmdk-sv"

	type $Props = {
		children: Snippet
	} & CommandPrimitive.EmptyProps

	let { children, class: className, ...props }: $Props = $props()
</script>

<CommandPrimitive.Empty class={cx("py-6 text-center text-sm", className)} {...props}>
	{@render children()}
</CommandPrimitive.Empty>
