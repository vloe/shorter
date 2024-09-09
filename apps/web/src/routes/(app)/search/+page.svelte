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

<div class="h-screen"></div>
<div class="sticky bottom-0 z-50 flex w-full py-6 lg:py-8">
	<SearchInput bind:value={params.q} oninput={handleInput} />
</div>
