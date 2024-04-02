<script lang="ts">
	import type { DomainArgs, DomainRes } from "$lib/types"
	import { createMutation } from "@tanstack/svelte-query"
	import { API_URL } from "$lib/constants"

	let domainArgs = $state<DomainArgs>({
		domain: "",
	})

	const domainMutation = createMutation({
		mutationFn: async () => {
			const res = await fetch(`${API_URL}/domain`, {
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify(domainArgs),
			})
			const data: DomainRes = await res.json()
			return data
		},
	})
</script>

<div class="flex w-min flex-col">
	<input bind:value={domainArgs.domain} />
	<button on:click={() => $domainMutation.mutate()}>click to send</button>
	{#if $domainMutation.data}
		{$domainMutation.data.domain_list}
	{/if}
</div>
