<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { apiUrl } from "$lib/constants/urls"
	import type { ShortenParams, ShortenRes } from "$lib/types/core"

	let params = $state<ShortenParams>({
		domain: "example.com",
	})

	let query = createQuery<ShortenRes, Error>({
		queryKey: ["shorten", params],
		queryFn: async () => {
			const res = await fetch(`${apiUrl}/shorten?domain=${params.domain}`, {
				method: "GET",
				headers: {
					"Content-Type": "application/json",
					"Accept": "application/json",
				},
			})
			if (!res.ok) throw new Error(await res.text())
			const data = await res.json()
			return data
		},
		retry: false,
	})
</script>

<input bind:value={params.domain} />
<button onclick={() => $query.refetch()}>click to refetch</button>
{#if $query.isError}
	<p class="text-red-300">{$query.error.message}</p>
{:else}
	<p class="text-green-300">{JSON.stringify($query.data)}</p>
{/if}
