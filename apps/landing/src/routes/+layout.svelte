<script lang="ts">
	import "../app.css"
	import type { Snippet } from "svelte"
	import Header from "$lib/components/Header.svelte"

	type $Props = {
		children: Snippet
	}

	let { children }: $Props = $props()
</script>

<div class="container mx-auto scroll-smooth px-8 sm:px-10 lg:px-12">
	<Header />
	{@render children()}
</div>
