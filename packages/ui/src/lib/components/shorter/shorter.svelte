<script lang="ts">
	import type { ShorterParams, ShorterRes } from "@sh/utils/src/bindings"

	import { dev } from "$app/environment"
	import { API_URL_DEV, API_URL_PROD } from "@sh/utils/src/constants"
	import { createQuery } from "@tanstack/svelte-query"

	import { Input } from "../input"
	import * as Table from "../table"

	let params: ShorterParams = $state({
		q: "",
	})

	let url = $derived(`${dev ? API_URL_DEV : API_URL_PROD}/shorter?q=${params.q}`)

	let query = $state(
		createQuery<ShorterRes, Error>(() => ({
			queryFn: async () => {
				const res = await fetch(url, {
					headers: {
						"Content-Type": "application/json",
					},
					method: "GET",
				})
				if (!res.ok) throw new Error(await res.text())
				const data = await res.json()
				return data
			},
			queryKey: ["shorter", params],
			retry: false,
		})),
	)
</script>

<div class="flex h-full flex-col gap-3" id="shorter">
	<Input
		bind:value={params.q}
		class="focus-visible:ring-0"
		on:input={() => query.refetch()}
		placeholder="example.com"
	/>

	{#if query.isSuccess && query.data}
		<Table.Root>
			<Table.Header>
				<Table.Row>
					<Table.Head>#</Table.Head>
					<Table.Head>domain</Table.Head>
					<Table.Head>status</Table.Head>
				</Table.Row>
			</Table.Header>
			<Table.Body>
				{#each query.data.domains as domain, i (i)}
					<Table.Row>
						<Table.Cell class="font-medium">{i + 1}</Table.Cell>
						<Table.Cell>{domain.name}</Table.Cell>
						<Table.Cell>
							{#if domain.available}
								<p class="text-green-400">available</p>
							{:else}
								<p class="text-red-400">unavailable</p>
							{/if}
						</Table.Cell>
					</Table.Row>
				{/each}
			</Table.Body>
		</Table.Root>
	{/if}
</div>
