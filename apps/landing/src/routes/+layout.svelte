<script lang="ts">
	import type { Snippet } from "svelte"

	import Header from "$lib/components/Header.svelte"
	import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query"

	import "../app.css"

	type $Props = {
		children: Snippet
	}

	let queryClient = new QueryClient()

	let { children }: $Props = $props()
</script>

<div class="sm:px:14 container mx-auto scroll-smooth px-6 lg:px-32">
	<QueryClientProvider client={queryClient}>
		<Header />
		{@render children()}
	</QueryClientProvider>
</div>
