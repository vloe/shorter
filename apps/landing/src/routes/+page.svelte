<script lang="ts">
	import WebGl from "$lib/components/WebGl.svelte"
	import { Btn } from "@sh/ui/src/lib/components/btn"
	import Arrow from "@sh/ui/src/lib/icons/Arrow.svelte"

	const title = "shorter | world's first domain shortener"
	const desc = "a domain shortener tool"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
	<meta content={desc} name="description" />
</svelte:head>

<WebGl />

<div class="pt-44">
	<div class="flex flex-col items-center gap-6 lg:gap-11">
		<h1
			class="text-center text-5xl font-light leading-[1.1] tracking-[-0.9px] sm:text-7xl sm:leading-none xl:text-[89px]"
		>
			world's first domain shortener
		</h1>
		<h2
			class="max-w-xl text-center text-base font-medium leading-normal text-primary/70 md:w-11/12 lg:text-lg"
		>
			searches through thousands of tlds to shorten your domain, for example linktree.com ->
			linktr.ee
		</h2>
		<div class="flex items-center gap-x-2">
			<Btn
				class="group h-8 gap-x-[5px] rounded-lg duration-200 ease-in-out hover:bg-primary/85"
				href="/register"
			>
				start now
				<Arrow />
			</Btn>
			<Btn
				class="!no-underline duration-200 ease-in-out hover:text-primary/85"
				href="https://github.com/vloe/shorter"
				intent="link"
				rel="noopener noreferrer"
				target="_blank"
			>
				star on github
			</Btn>
		</div>
	</div>
</div>
