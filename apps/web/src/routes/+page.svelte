<script lang="ts">
	import type { SearchParams } from "$lib/utils/bindings"

	import { goto } from "$app/navigation"
	import { SearchInput } from "$lib/components/ui/search-input"
	import { WaveBg } from "$lib/components/ui/wave-bg"

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

<WaveBg />

<section class="pt-28 lg:pt-44">
	<div class="container">
		<div class="flex flex-col items-center gap-6 lg:gap-8">
			<h1
				class="text-center text-5xl font-light tracking-[-0.9px] sm:text-6xl lg:text-7xl xl:text-[89px]"
			>
				world's first domain <br class="hidden sm:flex" /> shortener
			</h1>
			<h2 class="max-w-md text-center text-white/70 lg:text-lg">
				discover a shorter version of your domain, for example linktree.com -> linktr.ee
			</h2>
			<SearchInput bind:value={searchParams.q} class="max-w-xs" oninput={handleInput} />
		</div>
	</div>
</section>
<div class="h-screen"></div>
<div class="h-screen"></div>
