<script lang="ts">
	import type {
		DnsLookupParams,
		DnsLookupRes,
		SearchParams,
		SearchRes,
	} from "$lib/utils/bindings"

	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { Grid } from "$lib/components/icons/grid"
	import { List } from "$lib/components/icons/list"
	import { AsciiArt } from "$lib/components/ui/ascii-art"
	import { DomainCard } from "$lib/components/ui/domain-card"
	import { SearchBar } from "$lib/components/ui/search-bar"
	import { dnsLookup } from "$lib/queries/dnsLookup"
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

	let dnsLookupParams = $derived<DnsLookupParams>({
		q: searchQuery.data?.domains.map((d) => d.name) || [],
	})

	let dnsLookupQuery = createQuery<DnsLookupRes, Error>(() => ({
		enabled: searchQuery.isSuccess,
		queryFn: () => dnsLookup(dnsLookupParams),
		queryKey: ["dns-lookup", dnsLookupParams],
	}))

	function handleOnInput() {
		if (!browser) return
		$page.url.searchParams.set("q", searchParams.q)
		goto($page.url, { keepFocus: true, noScroll: true, replaceState: true })
	}

	let layout = $state((browser && localStorage.getItem("layout")) || "grid")
	let isGrid = $derived(layout === "grid")

	function toggleLayout() {
		if (!browser) return
		layout = isGrid ? "list" : "grid"
		localStorage.setItem("layout", layout)
	}

	const title = "search | shorter"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
</svelte:head>

{#if searchQuery.isSuccess}
	<main class="pb-[76px] pt-3 lg:pb-[88px] lg:pt-4">
		<div class="container mx-auto px-3">
			<div
				class={`grid grid-cols-1 gap-3 sm:gap-4 ${isGrid && "sm:grid-cols-2  lg:grid-cols-3 xl:grid-cols-4"}`}
			>
				{#each searchQuery.data.domains as domain}
					<DomainCard {dnsLookupQuery} {domain} />
				{/each}
			</div>
		</div>
	</main>
{:else if !searchQuery.isLoading && !searchQuery.isFetching}
	<AsciiArt
		class="absolute left-1/2 top-1/2 -z-50 -translate-x-1/2 -translate-y-1/2 transform pb-[64px] pt-3 text-[4px] lg:pb-[72px]"
	/>
{/if}

<div class="fixed bottom-0 z-50 w-full pb-6 lg:pb-8">
	<div class="container mx-auto px-3">
		<SearchBar bind:value={searchParams.q} oninput={handleOnInput}>
			<button class="hidden sm:flex" onclick={toggleLayout}>
				{#if isGrid}
					<List />
				{:else}
					<Grid />
				{/if}
			</button>
		</SearchBar>
	</div>
</div>
