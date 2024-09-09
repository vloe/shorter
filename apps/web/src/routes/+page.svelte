<script lang="ts">
	import type { SearchParams } from "$lib/utils/bindings"

	import { goto } from "$app/navigation"
	import { SearchInput } from "$lib/components/ui/search-input"

	let searchParams = $state<SearchParams>({
		q: "",
	})

	function handleInput() {
		searchParams.q && goto(`/search?q=${searchParams.q}`)
	}

	const title = "shorter | world's first domain shortener"
	const desc = "a domain shortener tool"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
	<meta content={desc} name="description" />
</svelte:head>

<main class="pt-40 lg:pt-60">
	<div class="container">
		<div class="flex flex-col items-center gap-6 lg:gap-8">
			<h1 class="text-center text-5xl tracking-[-1px] sm:text-6xl lg:text-7xl">
				world's first domain <br class="hidden sm:flex" /> shortener
			</h1>
			<h2 class="max-w-md text-center text-white/70 lg:text-lg">
				discover a shorter version of your domain, for example linktree.com -> linktr.ee
			</h2>
			<SearchInput bind:value={searchParams.q} class="max-w-xs" oninput={handleInput} />
		</div>
	</div>
</main>
