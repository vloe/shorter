<script lang="ts">
	import type { SearchParams } from "$lib/types/bindings"

	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { Header } from "$lib/components/ui/header"
	import { SearchBar } from "$lib/components/ui/search-bar"

	let params = $state<SearchParams>({
		q: "",
	})

	function handleOnInput() {
		const url = new URL(`${$page.url.origin}/search`)
		url.searchParams.append("q", params.q)
		goto(url.toString())
	}

	const title = "shorter"
	const desc = "a domain shortener tool"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
	<meta content={desc} name="description" />
</svelte:head>

<Header />

<div class="pt-24 xl:pt-44">
	<section class="flex flex-col items-center gap-6 lg:gap-8">
		<h1
			class="text-center text-5xl font-semibold leading-[1.1] tracking-[-1px] sm:text-6xl sm:leading-none lg:text-7xl"
		>
			search for shorter <br class="hidden min-[450px]:flex" /> domains
		</h1>
		<h2 class="max-w-md text-center text-white/75 lg:text-lg">
			discover shorter domain alternatives, for example linktree.com -> linktr.ee
		</h2>
		<SearchBar bind:value={params.q} class="max-w-xs" oninput={handleOnInput} />
	</section>
</div>
