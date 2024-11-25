<script lang="ts">
	import type { DnsLookupParams, SearchParams, SearchRes } from "$lib/types/bindings"

	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { Check } from "$lib/components/icons/check"
	import { Copy } from "$lib/components/icons/copy"
	import { Info } from "$lib/components/icons/info"
	import { Btn } from "$lib/components/ui/btn"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
	import { Input } from "$lib/components/ui/input"
	import * as Popover from "$lib/components/ui/popover"
	import { Skeleton } from "$lib/components/ui/skeleton"
	import { registrars } from "$lib/constants/registrars"
	import { dnsLookup } from "$lib/queries/dnsLookup"
	import { search } from "$lib/queries/search"
	import { createQueries, createQuery } from "@tanstack/svelte-query"

	let searchParams = $state<SearchParams>({
		q: (browser && $page.url.searchParams.get("q")) || "",
	})
	let searchQuery = createQuery<SearchRes, Error>(() => ({
		queryFn: () => search(searchParams),
		queryKey: ["search", searchParams],
		retry: false,
	}))

	let dnsLookupParams = $derived<DnsLookupParams[]>(
		searchQuery.data?.domains.map((domain) => ({ q: domain.name })) || [],
	)
	const dnsLookupQueries = createQueries({
		queries: () =>
			dnsLookupParams.map((params) => ({
				queryFn: () => dnsLookup(params),
				queryKey: ["dns-lookup", params],
			})),
	})

	let copied = $state("")
	function handleOnCopy(domain: string) {
		navigator.clipboard.writeText(domain)
		copied = domain
		setTimeout(() => {
			if (copied === domain) {
				copied = ""
			}
		}, 3000)
	}

	function handleOnInput() {
		if (!browser) return
		$page.url.searchParams.set("q", searchParams.q)
		goto($page.url, { keepFocus: true, noScroll: true, replaceState: true })
	}

	const title = "Search | Shorter"
	const desc = "Search for a shorter domain."
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
	<meta content={desc} name="description" />
</svelte:head>

<div class="pb-[88px] pt-3">
	<main class="flex flex-col gap-3">
		{#if searchQuery.isSuccess}
			{#each searchQuery.data.domains as domain, i}
				<div class="h-20 w-full rounded-lg border">
					<div class="flex h-full items-center justify-between px-6">
						<div class="flex items-center gap-x-2">
							<h3>{domain.name}</h3>
							<Popover.Root>
								<Popover.Trigger>
									<Info class="text-white/60" />
								</Popover.Trigger>
								<Popover.Content class="space-y-1 text-sm" side="top">
									<p>
										<span class="font-semibold">Type:</span>
										{domain.tld.category}
									</p>
									<p>
										<span class="font-semibold">Tld manager:</span>
										{domain.tld.manager}
									</p>
								</Popover.Content>
							</Popover.Root>
							<button onclick={() => handleOnCopy(domain.name)}>
								{#if copied === domain.name}
									<Check class="text-white/60" />
								{:else}
									<Copy class="text-white/60" />
								{/if}
							</button>
						</div>
						{#if dnsLookupQueries[i]?.isSuccess}
							<DropdownMenu.Root>
								<DropdownMenu.Trigger>
									{#snippet child({ props })}
										<Btn
											{...props}
											disabled={!dnsLookupQueries[i].data?.buyable}
										>
											Buy
										</Btn>
									{/snippet}
								</DropdownMenu.Trigger>
								<DropdownMenu.Content class="w-52" side="top">
									{#each registrars as { buyLink, icon, name, site }}
										<DropdownMenu.Item>
											{#snippet child({ props })}
												<a
													href={buyLink ? buyLink + domain.name : site}
													rel="noopener noreferrer"
													target="_blank"
													{...props}
												>
													<svelte:component this={icon} />
													{name}
												</a>
											{/snippet}
										</DropdownMenu.Item>
									{/each}
								</DropdownMenu.Content>
							</DropdownMenu.Root>
						{:else}
							<Skeleton class="h-7 w-14" />
						{/if}
					</div>
				</div>
			{/each}
		{/if}
	</main>
</div>

<div class="container fixed bottom-0 left-0 right-0 mx-auto px-6 pb-8 lg:px-16">
	<Input bind:value={searchParams.q} oninput={handleOnInput} />
</div>
