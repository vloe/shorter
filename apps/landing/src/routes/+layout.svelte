<script lang="ts">
	import type { Snippet } from "svelte"

	import Header from "$lib/components/Header.svelte"

	import "../app.css"

	type $Props = {
		children: Snippet
	}

	let { children }: $Props = $props()
</script>

<div class="container mx-auto scroll-smooth px-4 lg:px-16 xl:px-28">
	<Header />
	{@render children()}
</div>
