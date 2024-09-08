<script lang="ts">
	import type { Snippet } from "svelte"

	import { Header } from "$lib/components/ui/header"

	type $Props = {
		children: Snippet
	}

	let { children }: $Props = $props()
</script>

<div class="container mx-auto scroll-smooth px-6">
	<Header />
	{@render children()}
</div>
