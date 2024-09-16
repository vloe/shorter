<script lang="ts">
	import type { SearchParams, SearchRes } from "$lib/utils/bindings"

	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { SearchInput } from "$lib/components/ui/search-input"
	import { apiUrl } from "$lib/utils/urls"
	import { createQuery } from "@tanstack/svelte-query"

	let searchParams = $state<SearchParams>({
		q: (browser && $page.url.searchParams.get("q")) || "",
	})

	let searchQuery = createQuery<SearchRes, Error>(() => ({
		queryFn: async () => {
			const res = await fetch(`${apiUrl}/search${$page.url.search}`, {
				headers: {
					"Accept": "application/json",
					"Cache-Control": "max-age=300",
					"Content-Type": "application/json",
				},
				method: "GET",
			})
			if (!res.ok) throw new Error(await res.text())
			return await res.json()
		},
		queryKey: ["search", searchParams],
		retry: false,
	}))

	function handleInput() {
		$page.url.searchParams.set("q", searchParams.q)
		goto($page.url, { keepFocus: true, replaceState: true })
	}

	const title = "search | shorter"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
</svelte:head>

<main class="pb-[76px] pt-3 lg:pb-[88px] lg:pt-4">
	<div class="container mx-auto px-3">
		{#if searchQuery.isSuccess}
			<div
				class="grid grid-cols-1 gap-3 sm:grid-cols-2 sm:gap-4 lg:grid-cols-3 xl:grid-cols-4"
			>
				{#each searchQuery.data.domains as domain}
					<div
						class="flex h-24 select-none items-center justify-center rounded-md border p-6"
					></div>
				{/each}
			</div>
		{/if}
	</div>
</main>

<div class="fixed bottom-0 z-50 w-full pb-6 lg:pb-8">
	<div class="container mx-auto px-3">
		<SearchInput bind:value={searchParams.q} oninput={handleInput} />
	</div>
</div>
