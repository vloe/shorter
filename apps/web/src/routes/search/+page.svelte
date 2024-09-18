<script lang="ts">
	import type { SearchParams, SearchRes } from "$lib/utils/bindings"

	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { DomainCard } from "$lib/components/ui/domain-card"
	import { SearchInput } from "$lib/components/ui/search-input"
	import { search } from "$lib/queries/search"
	import { createQuery } from "@tanstack/svelte-query"

	let searchParams = $state<SearchParams>({
		q: (browser && $page.url.searchParams.get("q")) || "",
	})

	let searchQuery = createQuery<SearchRes, Error>(() => ({
		queryFn: () => search(searchParams),
		queryKey: ["search", searchParams],
		retry: false,
	}))

	function handleSearchInput() {
		$page.url.searchParams.set("q", searchParams.q)
		goto($page.url, { keepFocus: true, noScroll: true, replaceState: true })
	}

	const title = "search | shorter"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
</svelte:head>

<main class="pb-[76px] pt-3 lg:pb-[88px] lg:pt-4">
	<div class="container mx-auto px-3">
		<div class="grid grid-cols-1 gap-3 sm:grid-cols-2 sm:gap-4 lg:grid-cols-3 xl:grid-cols-4">
			{#if searchQuery.isSuccess}
				{#each searchQuery.data.domains as domain}
					<DomainCard {domain} />
				{/each}
			{/if}
		</div>
	</div>
</main>

<div class="fixed bottom-0 z-50 w-full pb-6 lg:pb-8">
	<div class="container mx-auto px-3">
		<SearchInput bind:value={searchParams.q} oninput={handleSearchInput} />
	</div>
</div>
