<script lang="ts">
	import type { Snippet } from "svelte"	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	type $Props = {
		children: Snippet
	} & HTMLAttributes<HTMLParagraphElement>

	let { children, class: className, ...props }: $Props = $props()
</script>

<p class={cx("text-sm text-white/75", className)} {...props}>
	{@render children()}
</p>
