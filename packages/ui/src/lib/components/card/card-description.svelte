<script lang="ts">
	import type { Snippet } from "svelte"
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "@sh/utils/src/cva.config"

	type $Props = {
		children: Snippet
	} & HTMLAttributes<HTMLParagraphElement>

	let { children, class: className, ...props }: $Props = $props()
</script>

<p class={cx("text-sm text-muted-foreground", className)} {...props}>
	{@render children()}
</p>
