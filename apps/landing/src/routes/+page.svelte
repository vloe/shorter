<script lang="ts">
	import { createQuery } from "@tanstack/svelte-query"
	import { apiUrl } from "$lib/constants/urls"
	import type { ShortenParams, ShortenRes } from "$lib/types/core"
	import Button from "@sh/ui/src/lib/components/Button.svelte"
	import Arrow from "@sh/ui/src/lib/icons/Arrow.svelte"
	import Github from "@sh/ui/src/lib/icons/Github.svelte"

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
			<h1 class="text-center text-5xl font-extrabold sm:text-7xl">a domain shortener tool</h1>
			<h2 class="my-6 max-w-2xl text-center">
				searches through thousands of tlds to find a shorter version of your domain, for
				example linktree.com --> linktr.ee
			</h2>
			<div class="flex items-center space-x-6">
				<Button class="group space-x-[5px] rounded-xl duration-200">
					<span>try now</span>
					<Arrow class="h-3 w-3 stroke-[1.2px]" />
				</Button>
				<a
					href="https://github.com/vloe/shorter"
					target="_blank"
					rel="noopener noreferrer"
					class="flex items-center space-x-1 text-sm font-medium duration-200 hover:text-primary/75"
				>
					<Github class="h-4 w-4" />
					<span>star on github</span>
				</a>
			</div>
		</main>
	</div>
</div>
