<script lang="ts">
	import type { ShortenArgs, ShortenRes } from "$lib/types"
	import { createMutation } from "@tanstack/svelte-query"
	import { API_URL } from "$lib/constants"

	let shortenArgs = $state<ShortenArgs>({ domain: "" })

	const shortenMutation = createMutation({
		mutationFn: async () => {
			const res = await fetch(`${API_URL}/shorten`, {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify(shortenArgs),
			})
			const data: ShortenRes = await res.json()
			console.log(data)
			return data
		},
	})
</script>

<div class="flex w-min flex-col">
	<input bind:value={shortenArgs.domain} />
	<button on:click={() => $shortenMutation.mutate()}>click to send</button>
	{#if $shortenMutation.data}
		{#each Object.entries($shortenMutation.data.domain_list[0]) as [key, value]}
			<p>{key}: {value}</p>
		{/each}
	{/if}
</div>
