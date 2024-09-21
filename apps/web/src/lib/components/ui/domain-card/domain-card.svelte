<script lang="ts">
	import type { Domain } from "$lib/utils/bindings"
	import type { DnsLookupRes } from "$lib/utils/bindings"
	import type { CreateQueryResult } from "@tanstack/svelte-query"

	import { Info } from "$lib/components/icons/info"
	import { Btn } from "$lib/components/ui/btn"
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
	<Btn class="w-20 flex-shrink-0 rounded-full">
		{#if dnsLookupQuery.isSuccess}
			{#if dnsLookupQuery.data.lookup[domain.name]}
				<span class="text-sm text-red-500">taken</span>
			{:else}
				<span class="text-sm text-green-500">buy</span>
			{/if}
		{:else}
			<Skeleton class="h-4 w-full" />
		{/if}
	</Btn>
</div>
