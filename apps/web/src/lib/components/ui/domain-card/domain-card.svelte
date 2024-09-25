<script lang="ts">
	import type { Domain } from "$lib/utils/bindings"
	import type { DnsLookupRes } from "$lib/utils/bindings"
	import type { CreateQueryResult } from "@tanstack/svelte-query"

	import { Info } from "$lib/components/icons/info"
	import { Select } from "$lib/components/icons/select"
	import { Button } from "$lib/components/ui/button"
	import * as Command from "$lib/components/ui/command"
	import * as Popover from "$lib/components/ui/popover"
	import { Skeleton } from "$lib/components/ui/skeleton"

	let search = $state("")

	type $Props = {
		dnsLookupQuery: CreateQueryResult<DnsLookupRes, Error>
		domain: Domain
	}

	let { dnsLookupQuery, domain }: $Props = $props()

	let links = [
		{
			href: "https://porkbun.com/checkout/search?q=",
			label: "porkbun",
		},
		{
			href: "https://www.namecheap.com/domains/registration/results/?domain=",
			label: "namecheap",
		},
		{
			href: "https://www.domain.com/registration/?search=",
			label: "domain.com",
		},
		{
			href: "https://godaddy.com/domainsearch/find?domainToCheck=",
			label: "godaddy",
		},
	]

	let shadow = $derived.by(() => {
		if (!dnsLookupQuery.isSuccess) return ""
		if (dnsLookupQuery.data.lookup[domain.name]) {
			return "shadow-red-600/50"
		} else {
			return "shadow-green-600"
		}
	})
</script>

<div
	class={`flex h-24 select-none items-center justify-between gap-x-1 rounded-lg border p-6 shadow-sm ${shadow}`}
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
	{#if dnsLookupQuery.isSuccess}
		<Popover.Root>
			<Popover.Trigger asChild let:builder>
				<Button
					builders={[builder]}
					class="h-7 flex-shrink-0 gap-x-0.5 rounded-full px-3.5"
					disabled={dnsLookupQuery.data.lookup[domain.name]}
				>
					buy
					<Select />
				</Button>
			</Popover.Trigger>
			<Popover.Content class="p-0">
				<Command.Root>
					<Command.Input
						bind:value={search}
						class="h-9"
						placeholder="search domain registrars..."
					/>
					{#key search}
						<Command.Group>
							{#each links as { href, label }}
								<Command.Item value={label}>
									<a
										class="h-full w-full"
										{href}
										rel="noopener noreferrer"
										target="_blank"
									>
										{label}
									</a>
								</Command.Item>
							{/each}
						</Command.Group>
					{/key}
				</Command.Root>
			</Popover.Content>
		</Popover.Root>
	{:else}
		<Skeleton class="h-7 w-[72px] flex-shrink-0 rounded-full" />
	{/if}
</div>
