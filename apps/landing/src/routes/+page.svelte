<script lang="ts">
	import { Button } from "@sh/ui/src/lib/components/button"
	import { Shorten } from "@sh/ui/src/lib/components/shorten"
	import { scrollTo } from "@sh/utils/src/scrollTo"
</script>

<div class="pt-24 xl:pt-40">
	<main class="flex flex-col">
		<div class="flex flex-col items-center gap-y-6">
			<h1 class="text-center text-7xl font-light">a domain shortener tool</h1>
			<h2 class="max-w-xl text-center text-muted-foreground">
				searches through thousands of tlds to find a shorter version of your domain, for
				example linktree.com -> linktr.ee
			</h2>
			<Button on:click={() => scrollTo("shorten")} class="rounded-xl">start now</Button>
		</div>
		<div class="mb-28 mt-64">
			<div class="flex w-full flex-col items-center">
				<div class="min-h-[450px] w-full max-w-3xl rounded-xl border border-input p-4">
					<Shorten />
				</div>
			</div>
		</div>
	</main>
</div>
