<script lang="ts">
	import { Input } from "@sh/ui/src/lib/components/input"

	const title = "shorter | world's first domain shortener"
	const desc = "a domain shortener tool"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
	<meta content={desc} name="description" />
</svelte:head>

<div class="pt-28 lg:pt-44">
	<div class="flex flex-col items-center gap-6 lg:gap-10">
		<h1
			class="text-center text-5xl font-thin tracking-[-0.9px] sm:text-6xl lg:text-7xl xl:text-[90px]"
		>
			world's first domain <br class="hidden sm:flex" /> shortener
		</h1>
		<h2 class="text-center text-primary/70 lg:text-lg">
			discover a shorter version of your domain, for example
			<br class="hidden xs:flex" />
			linktree.com -> linktr.ee
		</h2>
		<Input
			class="h-10 w-11/12 max-w-sm focus-visible:ring-primary/25 lg:h-11"
			placeholder="type any domain..."
		/>
	</div>
</div>
