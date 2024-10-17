<script lang="ts">
	import type { Snippet } from "svelte"

	import { browser } from "$app/environment"
	import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query"

	import "../app.css"

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
	<div class="container mx-auto max-w-5xl scroll-smooth px-6">
		{@render children()}
	</div>
</QueryClientProvider>
