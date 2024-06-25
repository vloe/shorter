<script lang="ts">
	import Button from "@sh/ui/src/lib/components/Button.svelte"
	import Arrow from "@sh/ui/src/lib/icons/Arrow.svelte"
	import Github from "@sh/ui/src/lib/icons/Github.svelte"
</script>

<div class="container px-6">
	<header class="flex h-20 items-center justify-between">
		<a href="/">
			<h2 class="font-lumiera text-3xl">shorter</h2>
		</a>
		<div class="flex items-center space-x-6">
			<Button class="group space-x-[5px] rounded-xl duration-200">
				<span>try for free</span>
				<Arrow class="h-3 w-3 stroke-[1.2px]" />
			</Button>
			<a
				href="https://github.com/vloe/shorter"
				target="_blank"
				rel="noopener noreferrer"
				class="flex items-center space-x-1.5 text-sm font-medium duration-200 hover:text-primary/75"
			>
				<Github />
				<span>star on github</span>
			</a>
		</div>
	</header>
</div>
