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

	$inspect($query.data)
</script>

<div class="container px-6">
	<main class="flex min-h-screen flex-col items-center justify-center"></main>
</div>
