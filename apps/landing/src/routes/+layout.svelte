<script lang="ts">
	import "../app.css"
	import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query"
	import { browser } from "$app/environment"
	import Header from "$lib/components/Header.svelte"
	import type { Snippet } from "svelte"

	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				enabled: browser,
			},
		},
	})

	type $Props = {
		children: Snippet
	}

	let { children }: $Props = $props()
</script>

<QueryClientProvider client={queryClient}>
	<div class="container px-6">
		<Header />
		{@render children()}
	</div>
</QueryClientProvider>
