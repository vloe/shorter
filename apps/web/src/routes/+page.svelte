<script lang="ts">
	import type { SearchParams } from "$lib/utils/bindings"

	import { goto } from "$app/navigation"
	import { SearchInput } from "$lib/components/ui/search-input"
	import { WebGL } from "$lib/components/ui/webgl"

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

<WebGL />

<main class="flex h-full min-h-screen items-center justify-center">
	<div class="container mx-auto px-6">
		<div class="flex flex-col items-center gap-6 lg:gap-8">
			<h1
				class="text-center text-5xl leading-[1.1] tracking-[-1px] sm:text-6xl sm:leading-none lg:text-7xl"
			>
				world's first domain <br class="hidden sm:flex" /> shortener
			</h1>
			<h2 class="max-w-md text-center text-white/75 lg:text-lg">
				discover a shorter version of your domain, for example linktree.com -> linktr.ee
			</h2>
			<SearchInput bind:value={searchParams.q} class="max-w-xs" oninput={handleInput} />
		</div>
	</div>
</main>
