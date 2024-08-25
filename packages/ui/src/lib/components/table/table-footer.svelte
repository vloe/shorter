<script lang="ts">
	import type { Snippet } from "svelte"
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "@sh/utils/src/cva.config"

	type $Props = {
		children: Snippet
	} & HTMLAttributes<HTMLTableSectionElement>

	let { children, class: className, ...props }: $Props = $props()
</script>

<tfoot class={cx("bg-primary font-medium text-primary-foreground", className)} {...props}>
	{@render children()}
</tfoot>
