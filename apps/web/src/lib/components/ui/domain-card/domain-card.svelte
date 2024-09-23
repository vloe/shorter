<script lang="ts">
	import type { Domain } from "$lib/utils/bindings"
	import type { DnsLookupRes } from "$lib/utils/bindings"
	import type { CreateQueryResult } from "@tanstack/svelte-query"

	import { Info } from "$lib/components/icons/info"
	import { Select } from "$lib/components/icons/select"
	import { Btn } from "$lib/components/ui/btn"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
	import * as Popover from "$lib/components/ui/popover"
	import { Skeleton } from "$lib/components/ui/skeleton"

	type $Props = {
		dnsLookupQuery: CreateQueryResult<DnsLookupRes, Error>
		domain: Domain
	}

	let { dnsLookupQuery, domain }: $Props = $props()

	let shadowClass = $derived.by(() => {
		if (!dnsLookupQuery.isSuccess) return ""

		if (dnsLookupQuery.data.lookup[domain.name]) {
			return "shadow-red-500/50"
		} else {
			return "shadow-green-500"
		}
	})
</script>

<div
	class={`flex h-24 select-none items-center justify-between gap-x-1 rounded-lg border p-6 shadow ${shadowClass}`}
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
		<DropdownMenu.Root>
			<DropdownMenu.Trigger asChild let:builder>
				<Btn
					builders={[builder]}
					class="flex-shrink-0 gap-x-1"
					disabled={dnsLookupQuery.data.lookup[domain.name]}
				>
					buy
					<Select />
				</Btn>
			</DropdownMenu.Trigger>
			<DropdownMenu.Content class="w-60">
				<DropdownMenu.Item
					href="https://porkbun.com/checkout/search?q={domain.name}"
					rel="noopener noreferrer"
					target="_blank"
				>
					porkbun
				</DropdownMenu.Item>
				<DropdownMenu.Item
					href="https://dash.cloudflare.com/?account=domains/register"
					rel="noopener noreferrer"
					target="_blank"
				>
					cloudflare
				</DropdownMenu.Item>
				<DropdownMenu.Item
					href="https://www.namecheap.com/domains/registration/results/?domain={domain.name}"
					rel="noopener noreferrer"
					target="_blank"
				>
					namecheap
				</DropdownMenu.Item>
				<DropdownMenu.Item
					href="https://www.domain.com/registration/?search={domain.name}"
					rel="noopener noreferrer"
					target="_blank"
				>
					domain.com
				</DropdownMenu.Item>
				<DropdownMenu.Item
					href="https://godaddy.com/domainsearch/find?domainToCheck={domain.name}"
					rel="noopener noreferrer"
					target="_blank"
				>
					godaddy
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	{:else}
		<Skeleton class="h-7 w-[72px] flex-shrink-0 rounded-full" />
	{/if}
</div>
