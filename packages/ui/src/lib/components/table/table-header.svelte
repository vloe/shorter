<script lang="ts">
	import type { Snippet } from "svelte"
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "@sh/utils/src/cva.config"

	type $Props = {
		children: Snippet
	} & HTMLAttributes<HTMLTableSectionElement>

	let { children, class: className, ...props }: $Props = $props()
</script>

<thead class={cx("[&_tr]:border-b", className)} {...props} on:click on:keydown>
	{@render children()}
</thead>
