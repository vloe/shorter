<script lang="ts">
	import { apiUrl } from "$lib/utils/urls"
	import { fetchEventSource } from "@microsoft/fetch-event-source"
	import { createQuery } from "@tanstack/svelte-query"

	let data = $state([])

	let query = createQuery(() => ({
		queryFn: async () => {
			await fetchEventSource(`${apiUrl().origin}/test`, {
				onmessage(ev) {
					data = [...data, ev.data]
					console.log(ev.data)
				},
			})
		},
		queryKey: ["test"],
	}))
</script>

<ul>
	{#each data as item}
		<li>{item}</li>
	{/each}
</ul>
