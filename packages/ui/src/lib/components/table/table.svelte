<script lang="ts">
	import type { Snippet } from "svelte"
	import type { HTMLTableAttributes } from "svelte/elements"

	import { cx } from "@sh/utils/src/cva.config"

	type $Props = {
		children: Snippet
	} & HTMLTableAttributes

	let { children, class: className, ...props }: $Props = $props()
</script>

<div class="relative w-full overflow-auto">
	<table class={cx("w-full caption-bottom text-sm", className)} {...props}>
		{@render children()}
	</table>
</div>
