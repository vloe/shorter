<script lang="ts">
	import type { HTMLInputAttributes } from "svelte/elements"

	import { Search } from "$lib/components/icons/search"
	import { cx } from "$lib/utils/cva.config"

	type $Props = HTMLInputAttributes

	let { class: className, value = $bindable(), ...props }: $Props = $props()
</script>

<div
	class={cx(
		"flex h-10 w-full items-center justify-center gap-2 rounded-full border bg-black px-3.5 py-1 shadow-sm",
		className,
	)}
>
	<Search class="text-white/40" />
	<input
		autocomplete="off"
		autofocus
		bind:value
		class="h-full w-full bg-transparent text-sm outline-none placeholder:text-white/40"
		placeholder="type any domain..."
		{...props}
	/>
</div>
