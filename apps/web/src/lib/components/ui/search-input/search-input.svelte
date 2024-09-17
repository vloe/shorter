<script lang="ts">
	import type { Snippet } from "svelte"
	import type { HTMLInputAttributes } from "svelte/elements"

	import { Search } from "$lib/components/icons/search"
	import { cx } from "$lib/utils/cva.config"

	type $Props = {
		children?: Snippet
	} & HTMLInputAttributes

	let { children, class: className, value = $bindable(), ...props }: $Props = $props()
</script>

<div
	class={cx(
		"flex h-10 w-full items-center justify-center gap-2 rounded-full border bg-black py-1.5 pl-3.5 pr-1.5 shadow-sm",
		className,
	)}
>
	<Search class="text-white/40" />
	<input
		autocomplete="off"
		autofocus
		bind:value
		class="h-full w-full bg-transparent outline-none placeholder:text-white/40"
		maxlength="32"
		placeholder="type any domain..."
		{...props}
	/>
	{#if children}
		{@render children()}
	{/if}
</div>
