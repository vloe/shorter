<script lang="ts">
	import { Button } from "@sh/ui/src/lib/components/button"
	import { Input } from "@sh/ui/src/lib/components/input"
	import * as Table from "@sh/ui/src/lib/components/table"
	import type { ShortenParams, ShortenRes } from "$lib/types/core"
	import { createQuery } from "@tanstack/svelte-query"
	import { apiUrl } from "$lib/constants/urls"
	import { scrollToTable } from "$lib/utils/scrollToTable"

	let params = $state<ShortenParams>({
		domain: "",
	})

	const query = createQuery<ShortenRes, Error>({
		queryKey: ["shorten", params],
		queryFn: async () => {
			const res = await fetch(`${apiUrl}/shorten?domain=${params.domain}`, {
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

<div class="pt-24 xl:pt-40">
	<main class="flex flex-col">
		<div class="flex flex-col items-center gap-y-6">
			<h1 class="text-center text-7xl font-light">a domain shortener tool</h1>
			<h2 class="max-w-xl text-center text-muted-foreground">
				searches through thousands of tlds to find a shorter version of your domain, for
				example linktree.com -> linktr.ee
			</h2>
			<Button on:click={() => scrollToTable()} class="rounded-xl">start now</Button>
		</div>
		<div class="mb-28 mt-64">
			<div class="flex w-full flex-col items-center">
				<div
					class="min-h-[450px] w-full max-w-3xl rounded-xl border border-input p-4"
					id="shorten-table"
				>
					<div class="mb-2 flex flex-row gap-x-2">
						<Input
							bind:value={params.domain}
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
							</Table.Row>
						</Table.Header>
						<Table.Body>
							{#if $query.isSuccess}
								{#each $query.data?.domains as domain, i (i)}
									<Table.Row>
										<Table.Cell>{i + 1}</Table.Cell>
										<Table.Cell>{domain.name}</Table.Cell>
									</Table.Row>
								{/each}
							{/if}
						</Table.Body>
					</Table.Root>
				</div>
			</div>
		</div>
	</main>
</div>
