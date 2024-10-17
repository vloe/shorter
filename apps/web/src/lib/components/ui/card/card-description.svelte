<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"
	import type { Snippet } from "svelte"

	type $Props = HTMLAttributes<HTMLParagraphElement> & {
		children: Snippet
	}

	let { children, class: className, ...props }: $Props = $props()
</script>

<p class={cx("text-sm text-white/75", className)} {...props}>
	{@render children()}
</p>
