<script lang="ts">
	import type { Snippet } from "svelte"
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	import type { HeadingLevel } from "./index.js"

	type $Props = {
		children: Snippet
		tag?: HeadingLevel
	} & HTMLAttributes<HTMLHeadingElement>

	let { children, class: className, tag = "h3", ...props }: $Props = $props()
</script>

<svelte:element
	this={tag}
	class={cx("font-semibold leading-none tracking-tight", className)}
	{...props}
>
	{@render children()}
</svelte:element>
