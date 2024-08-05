<script lang="ts">
	import "../app.css"
	import type { Snippet } from "svelte"
	import Header from "$lib/components/Header.svelte"
	import Wave from "$lib/components/Wave.svelte"

	type $Props = {
		children: Snippet
	}

	let { children }: $Props = $props()
</script>

<Wave />

<div class="container mx-auto scroll-smooth px-6 sm:px-14 lg:px-28">
	<Header />
	{@render children()}
</div>
