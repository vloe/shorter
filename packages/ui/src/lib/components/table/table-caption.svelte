<script lang="ts">
	import type { Snippet } from "svelte"
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "@sh/utils/src/cva.config"

	type $Props = {
		children: Snippet
	} & HTMLAttributes<HTMLTableCaptionElement>

	let { children, class: className, ...props }: $Props = $props()
</script>

<caption class={cx("mt-4 text-sm text-muted-foreground", className)} {...props}>
	{@render children()}
</caption>
