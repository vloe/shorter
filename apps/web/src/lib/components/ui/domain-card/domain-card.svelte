<script lang="ts">
	import type { Domain } from "$lib/utils/bindings"
	import type { DnsLookupRes } from "$lib/utils/bindings"
	import type { CreateQueryResult } from "@tanstack/svelte-query"

	import { Info } from "$lib/components/icons/info"
	import { Button } from "$lib/components/ui/button"
	import * as Popover from "$lib/components/ui/popover"
	import { Skeleton } from "$lib/components/ui/skeleton"

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
		{#if dnsLookupQuery.data.lookup[domain.name]}
			<Button class="h-7 flex-shrink-0 rounded-full" disabled>buy</Button>
		{:else}
			<Button
				class="h-7 flex-shrink-0 rounded-full"
				href={`https://porkbun.com/checkout/search?q=${domain.name}`}
				rel="noreferrer noopener"
				target="_blank"
			>
				buy
			</Button>
		{/if}
	{:else}
		<Skeleton class="h-7 w-16 rounded-full" />
	{/if}
</div>
