<script lang="ts">
	import type { SearchParams } from "$lib/utils/bindings"

	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { Search } from "$lib/components/icons/search"

	let params = $state<SearchParams>({
		q: (browser && $page.url.searchParams.get("q")) || "",
	})

	$effect(() => {
		if (browser) {
			$page.url.searchParams.set("q", params.q)
			goto($page.url, { replaceState: true })
		}
	})

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
		bind:value={params.q}
		class="h-full w-full bg-transparent text-sm outline-none placeholder:text-white/50 lg:text-base"
		placeholder="type any domain..."
	/>
</div>
