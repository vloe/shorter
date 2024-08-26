<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "@sh/utils/src/cva.config"
	import { Command as CommandPrimitive } from "cmdk-sv"

	type $Props = {
		children: Snippet
	} & CommandPrimitive.SeparatorProps

	let { children, class: className, ...props }: $Props = $props()
</script>

<CommandPrimitive.Separator class={cx("-mx-1 h-px bg-border", className)} {...props} />
