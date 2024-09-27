<script lang="ts">
	import type { SearchParams } from "$lib/utils/bindings"

	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { AsciiBg } from "$lib/components/ui/ascii-bg"
	import { SearchBar } from "$lib/components/ui/search-bar"

	let searchParams = $state<SearchParams>({
		q: "",
	})

	function handleOnInput() {
		if (!browser) return
		const url = new URL(`${$page.url.origin}/search`)
		url.searchParams.append("q", searchParams.q)
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

<AsciiBg />

<div class="pt-24 xl:pt-44">
	<section class="flex flex-col items-center gap-6 lg:gap-8">
		<h1
			class="text-center text-5xl font-medium leading-[1.1] tracking-[-0.9px] sm:text-6xl sm:leading-none lg:text-7xl"
		>
			search for shorter <br class="hidden min-[420px]:flex" /> domains
		</h1>
		<h2 class="max-w-md text-center text-primary/75 lg:text-lg">
			discover shorter versions of domains, for example linktree.com -> linktr.ee
		</h2>
		<SearchBar bind:value={searchParams.q} class="max-w-xs" oninput={handleOnInput} />
	</section>
</div>
