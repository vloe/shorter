<script lang="ts">
	import { Button } from "@sh/ui/src/lib/components/button"
	import { scrollTo } from "@sh/utils/src/scrollTo"
</script>

<header class="flex h-16 items-center justify-between">
	<a href="/">
		<h2 class="font-lumiera text-3xl">shorter</h2>
	</a>
	<Button on:click={() => scrollTo("shorten")} class="rounded-xl">start now</Button>
</header>
