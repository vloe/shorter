<script lang="ts">
	import { Input } from "../input"
	import { Button } from "../button"
	import * as Table from "../table"
	import * as Popover from "../popover"
	import Info from "../../icons/Info.svelte"
	import { createQuery } from "@tanstack/svelte-query"
	import { dev } from "$app/environment"
	import { API_URL_DEV, API_URL_PROD } from "@sh/constants/src/urls"
	import type { ShortenParams, ShortenRes } from "@sh/types/src/bindings"
	import { writable } from "svelte/store"

	let params = writable<ShortenParams>({
		domain: "",
	})

	$: url = `${dev ? API_URL_DEV : API_URL_PROD}/shorten?domain=${$params.domain}`
	const query = createQuery<ShortenRes, Error>({
		queryKey: ["shorten", params],
		queryFn: async () => {
			const res = await fetch(url, {
				method: "GET",
				headers: {
					"Content-Type": "application/json",
				},
			})
			if (!res.ok) throw new Error(await res.text())
			const data = await res.json()
			return data
		},
		enabled: false,
		retry: false,
	})
</script>

<div id="shorten" class="flex flex-col">
	<div class="mb-2 flex flex-row gap-x-2">
		<Input
			bind:value={$params.domain}
			on:keydown={(e) => e.key === "Enter" && $query.refetch()}
			placeholder="example.com"
		/>
		<Button on:click={() => $query.refetch()} class="h-9">shorten</Button>
	</div>
	<div>
		{#if $query.isError}
			<p class="ml-1 text-sm text-red-500">{$query.error.message}</p>
		{/if}
	</div>
	<Table.Root>
		<Table.Header>
			<Table.Row>
				<Table.Head class="w-[100px]">#</Table.Head>
				<Table.Head>domain</Table.Head>
				<Table.Head>status</Table.Head>
			</Table.Row>
		</Table.Header>
		<Table.Body>
			{#if $query.isSuccess}
				{#each $query.data?.domains as domain, i (i)}
					<Table.Row>
						<Table.Cell>{i + 1}</Table.Cell>
						<Table.Cell>
							<div class="flex items-center gap-1">
								<p>{domain.name}</p>
								<Popover.Root>
									<Popover.Trigger>
										<Info class="size-3.5 fill-muted-foreground" />
									</Popover.Trigger>
									<Popover.Content class="flex flex-col gap-1 text-sm">
										<div>
											<span class="font-medium">category:</span>
											<span>{domain.tld.category}</span>
										</div>
										<div>
											<span class="font-medium">manager:</span>
											<span>{domain.tld.manager.toLowerCase()}</span>
										</div>
									</Popover.Content>
								</Popover.Root>
							</div>
						</Table.Cell>
						<Table.Cell>
							<div class="flex items-center gap-1">
								<p>{domain.status.toLowerCase()}</p>
								<Popover.Root>
									<Popover.Trigger>
										<Info class="size-3.5 fill-muted-foreground" />
									</Popover.Trigger>
									<Popover.Content class="text-sm">
										<span class="font-medium">note:</span>
										<span>
											not 100% accurate because it only checks dns records
											(for now)
										</span>
									</Popover.Content>
								</Popover.Root>
							</div>
						</Table.Cell>
					</Table.Row>
				{/each}
			{/if}
		</Table.Body>
	</Table.Root>
</div>
