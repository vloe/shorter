<script lang="ts">
	import type { HTMLInputAttributes } from "svelte/elements"

	import { Search } from "$lib/components/icons/search"
	import { cx } from "$lib/utils/cva.config"

	type $Props = HTMLInputAttributes

	let { class: className, value = $bindable(), ...props }: $Props = $props()
</script>

<div class={cx("flex h-11 w-full items-center rounded-full border bg-black shadow-sm", className)}>
	<Search class="ml-4 size-[18px]" />
	<input
		autocomplete="off"
		autofocus
		class="ml-2 h-full w-full bg-transparent outline-none placeholder:text-white/50"
		maxlength="20"
		placeholder="type a domain..."
		{...props}
		bind:value
	/>
</div>
