<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "$lib/utils/cva.config"
	import { Button as ButtonPrimitive } from "bits-ui"

	type $Props = {
		children: Snippet
	} & ButtonPrimitive.Props

	let { builders, children, class: className, ...props }: $Props = $props()
</script>

<ButtonPrimitive.Root
	{builders}
	class={cx(
		"inline-flex h-7 w-20 items-center justify-center whitespace-nowrap rounded-full text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 disabled:pointer-events-none disabled:opacity-50",
		className,
	)}
	{...props}
	on:click
	on:keydown
>
	{@render children()}
</ButtonPrimitive.Root>
