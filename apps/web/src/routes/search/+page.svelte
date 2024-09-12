<script lang="ts">
	import type { SearchParams, SearchRes } from "$lib/utils/bindings"

	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { Info } from "$lib/components/icons/info"
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

	const title = "search | shorter"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
</svelte:head>

<main class="pb-20 pt-[72px] lg:pb-24 lg:pt-20">
	<div class="container">
		{#if searchQuery.isSuccess}
			<div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4">
				{#each searchQuery.data.domains as domain, i}
					<div
						class="flex h-24 select-none items-center justify-between rounded-lg border p-6"
					>
						<h3 class="flex items-center">
							{domain.sld}
							<span class="flex gap-x-px text-white/75">
								{domain.tldWithDot}
								<Popover.Root>
									<Popover.Trigger>
										<Info class="mb-1.5" />
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
							</span>
						</h3>
					</div>
				{/each}
			</div>
		{/if}
	</div>
</main>

<div class="fixed bottom-0 z-50 w-full pb-6 lg:pb-8">
	<div class="container">
		<SearchInput bind:value={searchParams.q} oninput={handleInput} />
	</div>
</div>
