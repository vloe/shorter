<script lang="ts">
	import type { Snippet } from "svelte"
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "@sh/utils/src/cva.config"

	type $Props = {
		"children": Snippet
		"data-state"?: unknown
	} & HTMLAttributes<HTMLTableRowElement>

	let { children, class: className, ...props }: $Props = $props()
</script>

<tr
	class={cx(
		"border-b transition-colors hover:bg-muted/50 data-[state=selected]:bg-muted",
		className,
	)}
	{...props}
	on:click
	on:keydown
>
	{@render children()}
</tr>
