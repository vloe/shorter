<script lang="ts">
	import { Search } from "$lib/components/icons/search"

	const title = "search | shorter"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
</svelte:head>

<div
	class="flex h-10 w-full items-center justify-center gap-2 rounded-full border border-white/25 px-3.5 py-1 shadow-sm lg:h-[46px]"
>
	<Search class="text-white/50 lg:size-4" />
	<input
		autofocus
		class="h-full w-full bg-transparent text-sm outline-none placeholder:text-white/50 lg:text-base"
		placeholder="type any domain..."
	/>
</div>
