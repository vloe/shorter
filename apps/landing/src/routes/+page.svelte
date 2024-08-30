<script lang="ts">
	import { dev } from "$app/environment"
	import { ArrowBtn } from "@sh/ui/src/lib/components/arrow-btn"
	import { Btn } from "@sh/ui/src/lib/components/btn"
	import { webUrl } from "@sh/utils/src/urls"

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
		<div class="flex items-center gap-x-1.5">
			<ArrowBtn
				class="h-8 rounded-lg duration-200 ease-in-out hover:bg-primary/85"
				href={`${webUrl(dev)}/register`}
			>
				start now
			</ArrowBtn>
			<Btn
				class="!no-underline duration-200 ease-in-out hover:text-primary/85"
				href={`${webUrl(dev)}/login`}
				intent="link"
			>
				login
			</Btn>
		</div>
	</div>
</div>
