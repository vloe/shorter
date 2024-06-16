<script lang="ts">
	import Github from "@sh/ui/src/lib/icons/Github.svelte"
	import X from "@sh/ui/src/lib/icons/X.svelte"
</script>

<div class="container px-6">
	<footer class="flex h-24 items-center justify-between">
		<div>
			<h2 class="font-lumiera text-3xl">shorter</h2>
			<p class="mt-1 text-sm text-muted-foreground">
				© {new Date().getFullYear()} vloe, inc.
			</p>
		</div>
		<div class="flex flex-row items-center space-x-4">
			<a
				href="https://github.com/vloe/shorter"
				target="_blank"
				rel="noopener noreferrer"
				class="transition-color text-primary duration-200 ease-in-out hover:text-primary/75"
			>
				<Github class="h-7 w-7" />
			</a>
			<a
				href="https://x.com/vloedev"
				target="_blank"
				rel="noopener noreferrer"
				class="transition-color text-primary duration-200 ease-in-out hover:text-primary/75"
			>
				<X class="h-6 w-6" />
			</a>
		</div>
	</footer>
</div>
