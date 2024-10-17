<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"
	import type { Snippet } from "svelte"

	type $Props = HTMLAttributes<HTMLDivElement> & {
		children: Snippet
	}

	let { children, class: className, ...props }: $Props = $props()
</script>

<div class={cx("flex flex-col space-y-1.5 p-6 pb-0", className)} {...props}>
	{@render children()}
</div>
