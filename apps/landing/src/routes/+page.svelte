<script lang="ts">
	import Button from "@sh/ui/src/lib/components/Button.svelte"
	import type { ShortenParams, ShortenRes } from "$lib/types/core"
	import { createQuery } from "@tanstack/svelte-query"
	import { apiUrl } from "$lib/constants/urls"

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
			<div class="flex items-center space-x-2">
				<Button class="rounded-xl">start now</Button>
				<Button
					href="https://github.com/vloe/shorter"
					target="_blank"
					rel="noreferrer noopener"
					variant="link"
					class="!no-underline hover:text-primary/75"
				>
					star on github
				</Button>
			</div>
		</div>
		<div class="my-44">
			<div class="flex w-full flex-col items-center">
				<div class="h-[450px] w-full max-w-3xl rounded-xl border border-input p-4">
					<div class="mb-2 flex flex-row gap-x-2">
						<input
							bind:value={params.domain}
							placeholder="example.com"
							class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50"
						/>
						<Button class="h-9" on:click={() => $query.refetch()}>shorten</Button>
					</div>
					{#if $query.isSuccess}
						<div class="flex flex-col gap-y-1">
							{#each $query.data?.domains as domain}
								<p class="ml-1 text-sm text-green-500">{domain}</p>
							{/each}
						</div>
					{:else}
						<p class="ml-1 text-sm text-red-500">{$query.error?.message}</p>
					{/if}
				</div>
			</div>
		</div>
	</main>
</div>
