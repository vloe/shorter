<script>
	import "../app.css"
	import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query"
	import Header from "$lib/components/Header.svelte"

	const queryClient = new QueryClient()

	let { children } = $props()
</script>

<QueryClientProvider client={queryClient}>
	<Header />
	{@render children()}
</QueryClientProvider>
