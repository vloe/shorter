<script lang="ts">
	import type { Snippet } from "svelte"
	import type { HTMLThAttributes } from "svelte/elements"

	import { cx } from "@sh/utils/src/cva.config"

	type $Props = {
		children: Snippet
	} & HTMLThAttributes

	let { children, class: className, ...props }: $Props = $props()
</script>

<th
	class={cx(
		"h-10 px-2 text-left align-middle font-medium text-muted-foreground [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]",
		className,
	)}
	{...props}
>
	{@render children()}
</th>
