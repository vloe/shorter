<script lang="ts">
	import type { Snippet } from "svelte"	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	type $Props = {
		children: Snippet
	} & HTMLAttributes<HTMLDivElement>

	let { children, class: className, ...props }: $Props = $props()
</script>

<div class={cx("p-6", className)} {...props}>
	{@render children()}
</div>
