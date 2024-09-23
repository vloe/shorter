<script lang="ts">
	import type { SearchParams } from "$lib/utils/bindings"
	import type { Snippet } from "svelte"

	import { Search } from "$lib/components/icons/search"
	import { cx } from "$lib/utils/cva.config"

	type $Props = {
		children?: Snippet
		class?: string
		onSearchInput: () => void
		searchParams: SearchParams
	}

	let { children, class: className, onSearchInput, searchParams = $bindable() }: $Props = $props()
</script>

<div
	class={cx(
		"flex h-10 w-full items-center gap-2 rounded-full border bg-black px-3.5 py-1.5 shadow-sm",
		className,
	)}
>
	<Search />
	<input
		autocomplete="off"
		autofocus
		bind:value={searchParams.q}
		class="h-full w-full bg-transparent outline-none placeholder:text-white/40"
		maxlength="30"
		oninput={onSearchInput}
		placeholder="type any domain..."
	/>
	{#if children}
		{@render children()}
	{/if}
</div>
