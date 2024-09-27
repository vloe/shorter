<script lang="ts">
	import type { HTMLTextareaAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	import type { TextareaEvents } from "./index.js"

	type $$Props = HTMLTextareaAttributes
	type $$Events = TextareaEvents

	let className: $$Props["class"] = undefined
	export let value: $$Props["value"] = undefined
	export { className as class }

	// Workaround for https://github.com/sveltejs/svelte/issues/9305
	// Fixed in Svelte 5, but not backported to 4.x.
	export let readonly: $$Props["readonly"] = undefined
</script>

<textarea
	bind:value
	class={cx(
		"flex min-h-[60px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50",
		className,
	)}
	on:blur
	on:change
	on:click
	on:focus
	on:input
	on:keydown
	on:keypress
	on:keyup
	on:mouseenter
	on:mouseleave
	on:mouseover
	on:paste
	{readonly}
	{...$$restProps}
></textarea>
