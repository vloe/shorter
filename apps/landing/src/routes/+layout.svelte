<script>
	import "../app.css"
	import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query"
	import { browser } from "$app/environment"
	import Header from "$lib/components/Header.svelte"

	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				enabled: browser,
			},
		},
	})

	let { children } = $props()
</script>

<QueryClientProvider client={queryClient}>
	<div class="container px-6">
		<Header />
		{@render children()}
	</div>
</QueryClientProvider>
