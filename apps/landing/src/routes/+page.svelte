<script lang="ts">
	import Button from "@sh/ui/src/lib/components/Button.svelte"
</script>

<div class="pt-24 xl:pt-40">
	<main class="flex flex-col items-center">
		<div class="flex flex-col items-center gap-y-6">
			<h1 class="text-center text-7xl font-light">a domain shortener tool</h1>
			<h2 class="max-w-xl text-center text-muted-foreground">
				searches through thousands of tlds to find a shorter version of your domain, for
				example linktree.com -> linktr.ee
			</h2>
			<div class="flex items-center space-x-2">
				<Button class="rounded-xl">start now</Button>
				<Button
					href="https://github.com/vloe/shorter"
					target="_blank"
					rel="noreferrer noopener"
					variant="link"
					class="!no-underline hover:text-primary/75"
				>
					star on github
				</Button>
			</div>
		</div>
	</main>
</div>
