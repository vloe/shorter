<script lang="ts">
	import { ArrowLink } from "$lib/components/arrow-link"
	import { Link } from "$lib/components/link"
	import { webUrl } from "$lib/utils/webUrl"

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
			<ArrowLink class="h-8 rounded-lg duration-200 ease-in-out" href={`${webUrl}/register`}>
				start now
			</ArrowLink>
			<Link class="duration-200 ease-in-out" href={`${webUrl}/login`} intent="secondary">
				login
			</Link>
		</div>
	</div>
</div>
