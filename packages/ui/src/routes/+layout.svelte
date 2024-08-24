<script lang="ts">
	import type { Snippet } from "svelte"

	import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query"

	import "../app.css"

	let queryClient = new QueryClient({})

	type $Props = {
		children: Snippet
	}

	let { children }: $Props = $props()
</script>

<div class="flex min-h-screen items-center justify-center">
	<QueryClientProvider client={queryClient}>
		{@render children()}
	</QueryClientProvider>
</div>
