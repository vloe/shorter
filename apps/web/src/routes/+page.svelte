<script lang="ts">
	import type { DomainArgs, DomainRes } from "$lib/types"
	import { createMutation } from "@tanstack/svelte-query"

	let domainArgs = $state<DomainArgs>({
		domain: "",
	})

	let domainMutation = $derived(
		createMutation({
			mutationKey: ["domain"],
			mutationFn: async (domainArgs: DomainArgs) => {
				const res = await fetch("https://api.shorter.dev/domain", {
					method: "POST",
					headers: {
						"Content-Type": "application/json",
					},
					body: JSON.stringify(domainArgs),
				})
				const data: DomainRes = await res.json()
				return data
			},
		}),
	)
</script>

<input bind:value={domainArgs.domain} />
<button>click to send</button>
