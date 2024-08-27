<script lang="ts">
	import type { Snippet } from "svelte"
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "@sh/utils/src/cva.config"

	type $Props = {
		children: Snippet
		logo: boolean
	} & HTMLAttributes<HTMLDivElement>

	let { children, class: className, ...props }: $Props = $props()
</script>

<div class={cx("rounded-xl border bg-card text-card-foreground shadow", className)} {...props}>
	{@render children()}
</div>
