<script lang="ts">
	import type { Snippet } from "svelte"
	import type { HTMLTdAttributes } from "svelte/elements"

	import { cx } from "@sh/utils/src/cva.config"

	type $Props = {
		children: Snippet
	} & HTMLTdAttributes

	let { children, class: className, ...props }: $Props = $props()
</script>

<td
	class={cx(
		"p-2 align-middle [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]",
		className,
	)}
	{...props}
	on:click
	on:keydown
>
	{@render children()}
</td>
