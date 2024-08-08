<script lang="ts">
	import type { Snippet } from "svelte"
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "@sh/utils/src/cva.config"

	type $Props = {
		children: Snippet
	} & HTMLAttributes<HTMLSpanElement>

	let { children, class: className, ...props }: $Props = $props()
</script>

<span class={cx("ml-auto text-xs tracking-widest opacity-60", className)} {...props}>
	{@render children()}
</span>
