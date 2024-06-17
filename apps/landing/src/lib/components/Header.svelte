<script lang="ts">
	import Button from "@sh/ui/src/lib/components/Button.svelte"
	import Arrow from "@sh/ui/src/lib/icons/Arrow.svelte"
</script>

<div class="container px-6">
	<header class="flex h-20 items-center justify-between">
		<a href="/">
			<h2 class="font-lumiera text-3xl">shorter</h2>
		</a>
		<div class="flex items-center space-x-6">
			<Button class="group space-x-[5px] rounded-xl duration-200">
				<span>start now</span>
				<Arrow class="h-3 w-3 stroke-[1.2px]" />
			</Button>
			<a
				href="/"
				class="group flex items-center space-x-[5px] text-sm font-medium duration-200 hover:text-primary/75"
			>
				<span>log in</span>
				<Arrow class="h-3 w-3 stroke-[1.2px]" />
			</a>
		</div>
	</header>
</div>
