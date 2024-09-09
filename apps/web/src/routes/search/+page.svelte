<script lang="ts">
	import type { SearchParams, SearchRes } from "$lib/utils/bindings"

	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { SearchInput } from "$lib/components/ui/search-input"
	import { apiUrl } from "$lib/utils/urls"
	import { createQuery } from "@tanstack/svelte-query"

	let params = $state<SearchParams>({
		q: (browser && $page.url.searchParams.get("q")) || "",
	})

	function handleInput() {
		$page.url.searchParams.set("q", params.q)
		goto($page.url, { keepFocus: true, noScroll: true, replaceState: true })
	}

	let query = createQuery<SearchRes, Error>(() => ({
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
			const data = await res.json()
			return data
		},
		queryKey: ["search", params],
		retry: false,
	}))

	const title = "search | shorter"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
</svelte:head>

<main class="pb-20 pt-[72px] lg:pb-24 lg:pt-20">
	<div class="container">
		{#if query.isSuccess && query.data}
			<div
				class="grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 lg:gap-6"
			>
				{#each query.data.domains as domain}
					<div class="flex h-24 items-center justify-center rounded-lg border">
						<span class="text-center text-lg font-medium">{domain.name}</span>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</main>

<div class="fixed bottom-0 z-50 w-full pb-6 lg:pb-8">
	<div class="container">
		<SearchInput bind:value={params.q} oninput={handleInput} />
	</div>
</div>
