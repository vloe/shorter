<script lang="ts">
	import type { SearchParams, SearchRes } from "$lib/utils/bindings"

	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { Grid } from "$lib/components/icons/grid"
	import { Info } from "$lib/components/icons/info"
	import { List } from "$lib/components/icons/list"
	import { Btn } from "$lib/components/ui/btn"
	import * as Popover from "$lib/components/ui/popover"
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

<main class="pb-[76px] pt-3 lg:pb-[88px] lg:pt-4">
	<div class="container mx-auto px-3">
		{#if searchQuery.isSuccess}
			<div
				class={`grid grid-cols-1 gap-3 sm:gap-4 ${isGrid && "sm:grid-cols-2  lg:grid-cols-3 xl:grid-cols-4"}`}
			>
				{#each searchQuery.data.domains as domain}
					<div
						class="flex h-24 select-none items-center justify-between gap-x-1 rounded-lg border p-6"
					>
						<h3 class="flex min-w-0 items-center">
							<span class="flex min-w-0 items-center">
								<span class="overflow-hidden">{domain.sld}</span>
								<span class="flex-shrink-0 text-white/75">{domain.tldWithDot}</span>
							</span>
							<Popover.Root>
								<Popover.Trigger>
									<Info class="mb-2 ml-px flex-shrink-0 text-white/75" />
								</Popover.Trigger>
								<Popover.Content class="flex flex-col space-y-2 p-3 text-sm">
									<p>
										<span class="font-semibold">type:</span>
										{domain.tldInfo.category}
									</p>
									<p>
										<span class="font-semibold">manager:</span>
										{domain.tldInfo.manager}
									</p>
								</Popover.Content>
							</Popover.Root>
						</h3>
						<Btn class="flex-shrink-0 rounded-full">
							{#if domain.isRegistered}
								<span class="text-red-500">unavailable</span>
							{:else}
								<span class="text-green-500">available</span>
							{/if}
						</Btn>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</main>

<div class="fixed bottom-0 z-50 w-full pb-6 lg:pb-8">
	<div class="container mx-auto px-3">
		<SearchInput bind:value={searchParams.q} oninput={handleInput}>
			<Btn class="h-full rounded-l-[8rem] rounded-r-[32rem] px-1.5" onclick={toggleLayout}>
				{#if isGrid}
					<List />
				{:else}
					<Grid />
				{/if}
			</Btn>
		</SearchInput>
	</div>
</div>
