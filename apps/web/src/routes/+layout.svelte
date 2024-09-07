<script lang="ts">
	import type { Snippet } from "svelte"

	import { Header } from "$lib/components/ui/header"

	import "../app.css"

	type $Props = {
		children: Snippet
	}

	let { children }: $Props = $props()
</script>

<div class="relative scroll-smooth">
	<Header />
	{@render children()}
</div>
