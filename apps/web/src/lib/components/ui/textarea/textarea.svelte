<script lang="ts">
	import type { HTMLTextareaAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	type $Props = HTMLTextareaAttributes

	let { class: className, value = $bindable(), ...props }: $Props = $props()
</script>

<textarea
	bind:value
	class={cx(
		"flex min-h-[60px] w-full rounded-lg border bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-white/50 focus-visible:outline-none focus-visible:ring-0 disabled:cursor-not-allowed disabled:opacity-50",
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
	{...props}
></textarea>
