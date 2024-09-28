<script lang="ts">
	import type { Domain } from "$lib/utils/bindings"
	import type { DnsLookupRes } from "$lib/utils/bindings"
	import type { CreateQueryResult } from "@tanstack/svelte-query"

	import { Info } from "$lib/components/icons/info"
	import { Button } from "$lib/components/ui/button"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
	import * as Popover from "$lib/components/ui/popover"
	import { Skeleton } from "$lib/components/ui/skeleton"
	import { registrars } from "$lib/constants/registrars"

	type $Props = {
		dnsLookupQuery: CreateQueryResult<DnsLookupRes, Error>
		domain: Domain
	}

	let { dnsLookupQuery, domain }: $Props = $props()
</script>

<div class="flex h-24 select-none items-center justify-between gap-x-1 rounded-lg border p-6">
	<h3 class="flex min-w-0 items-center">
		<span class="flex min-w-0 items-center">
			<span class="overflow-hidden">{domain.sld}</span>
			<span class="flex-shrink-0 text-primary/85">{domain.tldWithDot}</span>
		</span>
		<Popover.Root>
			<Popover.Trigger>
				<Info class="mb-2 ml-px flex-shrink-0 text-primary/85" />
			</Popover.Trigger>
			<Popover.Content class="space-y-1 rounded-lg text-sm">
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
		<DropdownMenu.Root>
			<DropdownMenu.Trigger asChild let:builder>
				<Button
					builders={[builder]}
					class="h-7 flex-shrink-0 rounded-full"
					disabled={dnsLookupQuery.data.lookup[domain.name]}
				>
					buy
				</Button>
			</DropdownMenu.Trigger>
			<DropdownMenu.Content class="w-52 rounded-lg">
				{#each registrars as { buyLink, icon, name, site }}
					<DropdownMenu.Item
						class="items-center gap-x-3 rounded-md"
						href={buyLink ? buyLink + domain.name : site}
						rel="noopener noreferrer"
						target="_blank"
					>
						<svelte:component this={icon} />
						{name}
					</DropdownMenu.Item>
				{/each}
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	{:else}
		<Skeleton class="h-7 w-16 rounded-full" />
	{/if}
</div>
