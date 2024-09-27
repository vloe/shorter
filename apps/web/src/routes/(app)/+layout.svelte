<script lang="ts">
	import type { Snippet } from "svelte"

	import { browser } from "$app/environment"
	import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query"

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
	{@render children()}
</QueryClientProvider>
