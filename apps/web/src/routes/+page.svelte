<script lang="ts">
	import type { SearchParams } from "$lib/utils/bindings"

	import { browser } from "$app/environment"
	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { AsciiArt } from "$lib/components/ui/ascii-art"
	import { SearchBar } from "$lib/components/ui/search-bar"
	import { asciiArtPos } from "$lib/constants/asciiArtPos"

	let searchParams = $state<SearchParams>({
		q: "",
	})

	function onSearchInput() {
		if (!browser && !searchParams.q) return
		const url = new URL(`${$page.url.origin}/search`)
		url.searchParams.append("q", searchParams.q)
		goto(url.toString())
	}

	const title = "shorter | search for shorter domains"
	const desc = "a domain shortener tool"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
	<meta content={desc} name="description" />
</svelte:head>

<main class="relative flex h-full min-h-screen items-center justify-center overflow-hidden">
	{#each asciiArtPos as { rotation, scale, x, y }}
		<div
			class="absolute"
			style="left: {x}%; top: {y}%; transform: rotate({rotation}deg) scale({scale});"
		>
			<AsciiArt class="text-[1px]" />
		</div>
	{/each}

	<div class="container relative z-10 mx-auto px-6">
		<div class="flex flex-col items-center gap-6 lg:gap-8">
			<h1
				class="text-center text-5xl leading-[1.1] tracking-[-0.9px] sm:text-6xl sm:leading-none lg:text-7xl"
			>
				search for shorter <br class="hidden min-[400px]:flex" /> domains
			</h1>
			<h2 class="max-w-md text-center text-white/75 lg:text-lg">
				discover shorter versions of a domain, for example linktree.com -> linktr.ee
			</h2>
			<SearchBar bind:searchParams class="max-w-xs" {onSearchInput} />
		</div>
	</div>
</main>
