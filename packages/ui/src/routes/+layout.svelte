<script>
	import "../app.css"
	import { QueryClient, QueryClientProvider } from "@tanstack/svelte-query"
	import { browser } from "$app/environment"


	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				enabled: browser,
			},
		},
	})
</script>


<QueryClientProvider client={queryClient}>
    <div class="flex items-center justify-center min-h-screen">
        <slot />
    </div>
</QueryClientProvider>
