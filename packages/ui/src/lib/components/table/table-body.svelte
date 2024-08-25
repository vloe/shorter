<script lang="ts">
	import type { Snippet } from "svelte"
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "@sh/utils/src/cva.config"

	type $Props = {
		children: Snippet
	} & HTMLAttributes<HTMLTableSectionElement>

	let { children, class: className, ...props }: $Props = $props()
</script>

<tbody class={cx("[&_tr:last-child]:border-0", className)} {...props}>
	{@render children()}
</tbody>
