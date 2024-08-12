<script lang="ts">
	import type { HTMLInputAttributes } from "svelte/elements"

	import { cx } from "@sh/utils/src/cva.config"

	type $Props = HTMLInputAttributes

	let { class: className, value = $bindable(), ...props }: $Props = $props()
</script>

<input
	bind:value
	class={cx(
		"flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50",
		className,
	)}
	on:blur
	on:change
	on:click
	on:focus
	on:focusin
	on:focusout
	on:input
	on:keydown
	on:keypress
	on:keyup
	on:mouseenter
	on:mouseleave
	on:mousemove
	on:mouseover
	on:paste
	on:wheel|passive
	{...props}
/>
