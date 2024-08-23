<script lang="ts">
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
		<div class="flex items-center gap-x-1.5">
			<Btn
				class="group h-8 gap-x-[5px] rounded-lg transition-all duration-200 ease-in-out hover:bg-primary/85"
				href="/"
			>
				start now
				<Arrow />
			</Btn>
			<Btn
				class="disabled !no-underline transition-all duration-200 ease-in-out hover:text-white/85"
				href="/"
				intent="link"
			>
				login
			</Btn>
		</div>
	</div>
</div>
