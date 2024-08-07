<script lang="ts">
<<<<<<< HEAD
	import type { Snippet } from "svelte"

	import Header from "$lib/components/Header.svelte"

	import "../app.css"
=======
	import "../app.css"
	import type { Snippet } from "svelte"
	import Header from "$lib/components/Header.svelte"
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c

	type $Props = {
		children: Snippet
	}

	let { children }: $Props = $props()
</script>

<div class="container mx-auto scroll-smooth px-4 lg:px-16 xl:px-28">
	<Header />
	{@render children()}
</div>
