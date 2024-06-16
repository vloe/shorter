<script lang="ts">
	import Button from "@sh/ui/src/lib/components/Button.svelte"
	import Arrow from "@sh/ui/src/lib/icons/Arrow.svelte"
</script>

<div class="container px-6">
	<header class="flex h-20 items-center justify-between">
		<a href="/">
			<h2 class="font-lumiera text-3xl">shorter</h2>
		</a>
		<Button class="group space-x-1.5 rounded-xl duration-200">
			<span>try for free</span>
			<Arrow class="h-3 w-3 stroke-[1.3px]" />
		</Button>
	</header>
</div>
