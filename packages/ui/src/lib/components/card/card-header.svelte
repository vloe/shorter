<script lang="ts">
	import type { Snippet } from "svelte"
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "@sh/utils/src/cva.config"

	type $Props = {
		children: Snippet
	} & HTMLAttributes<HTMLDivElement>

	let { children, class: className, ...props }: $Props = $props()
</script>

<div class={cx("flex flex-col space-y-1.5 p-6", className)} {...props}>
	{@render children()}
</div>
