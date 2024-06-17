<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { apiUrl } from "$lib/constants/urls"
	import type { ShortenParams, ShortenRes } from "$lib/types/core"
	import Button from "@sh/ui/src/lib/components/Button.svelte"
	import Arrow from "@sh/ui/src/lib/icons/Arrow.svelte"

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
	<div class="pt-24 xl:pt-44">
		<main class="flex min-h-screen flex-col items-center">
			<h1 class="text-center text-5xl font-extrabold sm:text-7xl">
				worlds first
				<span class="text-muted-foreground line-through decoration-4">url</span>
				domain
				<br class="hidden lg:flex" /> shortener
			</h1>
			<h2 class="my-6 text-center">
				find a shorter version of your domain like linktree.com -> linktr.ee.
			</h2>
			<Button class="group space-x-1.5 rounded-xl duration-200">
				<span>start now</span>
				<Arrow class="h-3 w-3 stroke-[1.3px]" />
			</Button>
		</main>
	</div>
</div>
