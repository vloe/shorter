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
		"inline-flex h-7 items-center justify-center whitespace-nowrap rounded-full bg-white px-4 py-2 text-sm font-medium text-black shadow transition-colors hover:bg-white/85 disabled:pointer-events-none disabled:opacity-50",
		className,
	)}
	{...props}
	on:click
	on:keydown
>
	{@render children()}
</ButtonPrimitive.Root>
