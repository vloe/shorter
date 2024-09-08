<script lang="ts">
	import type { SearchParams, SearchRes } from "$lib/utils/bindings"

	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { Search } from "$lib/components/icons/search"
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
<div class="h-screen"></div>
<div class="sticky bottom-6 z-50 w-full lg:bottom-14">
	<div
		class="flex h-10 w-full items-center justify-center gap-2 rounded-full border bg-black px-3.5 py-1 shadow-sm lg:h-[46px]"
	>
		<Search class="text-white/50 lg:size-4" />
		<input
			autocomplete="off"
			autofocus
			bind:value={params.q}
			class="h-full w-full bg-transparent text-sm outline-none placeholder:text-white/50 lg:text-base"
			oninput={handleInput}
			placeholder="type any domain..."
		/>
	</div>
</div>
