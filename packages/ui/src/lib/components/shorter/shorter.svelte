<script lang="ts">
	import type { ShorterParams, ShorterRes } from "@sh/utils/src/bindings"

	import { dev } from "$app/environment"
	import { API_URL_DEV, API_URL_PROD } from "@sh/utils/src/constants"
	import { createQuery } from "@tanstack/svelte-query"

	import { Input } from "../input"

	let params: ShorterParams = $state({
		q: "",
	})

	let query = $state(
		createQuery<ShorterRes, Error>(() => ({
			queryFn: async () => {
				let url = `${dev ? API_URL_DEV : API_URL_PROD}/shorter?q=${params.q}`
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

<div class="flex h-full flex-col" id="shorter">
	<Input
		bind:value={params.q}
		class="focus-visible:ring-0"
		on:input={() => query.refetch()}
		placeholder="example.com"
	/>
</div>
