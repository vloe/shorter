<script lang="ts">
	import type { ShorterParams } from "@sh/utils/src/bindings"

	import { goto } from "$app/navigation"
	import { page } from "$app/stores"
	import { Input } from "@sh/ui/src/lib/components/input"

	let params: ShorterParams = $state({
		q: $page.url.searchParams.get("q") || "",
	})

	$effect(() => {
		if (params.q) {
			$page.url.searchParams.set("q", params.q)
		} else {
			$page.url.searchParams.delete("q")
		}
		goto($page.url, { keepFocus: true, noScroll: true })
	})
</script>

<Input
	bind:value={params.q}
	class="h-10 w-11/12 max-w-sm focus-visible:ring-primary/25 lg:h-11"
	placeholder="type any domain..."
/>
