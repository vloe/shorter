<script lang="ts">
	import type { SearchParams } from "$lib/types/bindings"

	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { Input } from "$lib/components/ui/input"

	let params = $state<SearchParams>({
		q: "",
	})

	function handleOnInput() {
		const url = new URL(`${$page.url.origin}/search`)
		url.searchParams.append("q", params.q)
		goto(url.toString())
	}

	const title = "Shorter"
	const desc = "A domain shortener tool."
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
	<meta content={desc} name="description" />
</svelte:head>

<div class="pt-24 lg:pt-44">
	<section class="flex flex-col items-center gap-6 lg:gap-8">
		<h1
			class="text-center font-neue text-5xl leading-[1.1] tracking-[-1px] sm:text-6xl sm:leading-none lg:text-7xl xl:text-[86px]"
		>
			Search for shorter <br /> domains
		</h1>
		<h2 class="max-w-sm text-center text-white/75 lg:text-lg">
			Discover shorter versions of your domain like linktree.com -> linktr.ee.
		</h2>
		<Input bind:value={params.q} class="max-w-xs" oninput={handleOnInput} />
	</section>
</div>
