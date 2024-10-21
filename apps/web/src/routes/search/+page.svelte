<script lang="ts">
	import type {
		DnsLookupParams,
		DnsLookupRes,
		SearchParams,
		SearchRes,
	} from "$lib/types/bindings"

	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { Check } from "$lib/components/icons/check"
	import { Copy } from "$lib/components/icons/copy"
	import { Info } from "$lib/components/icons/info"
	import { Btn } from "$lib/components/ui/btn"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
	import * as Popover from "$lib/components/ui/popover"
	import { SearchBar } from "$lib/components/ui/search-bar"
	import { Skeleton } from "$lib/components/ui/skeleton"
	import { registrars } from "$lib/constants/registrars"
	import { dnsLookup } from "$lib/queries/dnsLookup"
	import { search } from "$lib/queries/search"
	import { createQuery } from "@tanstack/svelte-query"

	let copiedDomain = $state()

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

	function handleOnCopy(domain: string) {
		navigator.clipboard.writeText(domain)
		copiedDomain = domain
		setTimeout(() => {
			if (copiedDomain === domain) {
				copiedDomain = ""
			}
		}, 3000)
	}

	const title = "search | shorter"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
</svelte:head>

<div class="pb-[88px] pt-3">
	<main class="flex flex-col gap-3">
		{#if searchQuery.isSuccess}
			{#each searchQuery.data.domains as domain}
				<div class="h-20 w-full rounded-lg border">
					<div class="flex h-full items-center justify-between px-6">
						<div class="flex items-center gap-x-2">
							<h3 class="font-medium">{domain.name}</h3>
							<Popover.Root>
								<Popover.Trigger>
									<Info class="text-white/60" />
								</Popover.Trigger>
								<Popover.Content class="space-y-1 text-sm" side="top">
									<p>
										<span class="font-semibold">type:</span>
										{domain.tld.category}
									</p>
									<p>
										<span class="font-semibold">tld manager:</span>
										{domain.tld.manager}
									</p>
								</Popover.Content>
							</Popover.Root>
							<button onclick={() => handleOnCopy(domain.name)}>
								{#if copiedDomain === domain.name}
									<Check class="text-white/60" />
								{:else}
									<Copy class="text-white/60" />
								{/if}
							</button>
						</div>
						{#if dnsLookupQuery.isSuccess}
							<DropdownMenu.Root>
								<DropdownMenu.Trigger asChild let:builder>
									<Btn
										builders={[builder]}
										disabled={dnsLookupQuery.data.lookup[domain.name]}
									>
										buy
									</Btn>
								</DropdownMenu.Trigger>
								<DropdownMenu.Content class="w-52" side="top">
									{#each registrars as { buyLink, icon, name, site }}
										<DropdownMenu.Item
											href={buyLink ? buyLink + domain.name : site}
											rel="noopener noreferrer"
											target="_blank"
										>
											<svelte:component this={icon} />
											<span class="ml-2.5">{name}</span>
										</DropdownMenu.Item>
									{/each}
								</DropdownMenu.Content>
							</DropdownMenu.Root>
						{:else}
							<Skeleton class="h-8 w-14 rounded-lg" />
						{/if}
					</div>
				</div>
			{/each}
		{/if}
	</main>
</div>

<div class="fixed bottom-0 left-0 right-0 flex justify-center px-6 pb-8">
	<SearchBar bind:value={searchParams.q} class="max-w-[976px]" oninput={handleOnInput} />
</div>
