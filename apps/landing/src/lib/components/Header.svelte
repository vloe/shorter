<script lang="ts">
	import { Button } from "@sh/ui/src/lib/components/button"
	import { scrollToElement } from "$lib/utils/scrollToElement"
</script>

<header class="flex h-16 items-center justify-between">
	<a href="/">
		<h2 class="font-lumiera text-3xl">shorter</h2>
	</a>
	<Button on:click={() => scrollToElement("shorten-table")} class="rounded-xl">start now</Button>
</header>
