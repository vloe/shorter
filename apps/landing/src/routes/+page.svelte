<script lang="ts">
	import Wave from "$lib/components/Wave.svelte"
	import Arrow from "$lib/icons/Arrow.svelte"
	import { Btn } from "$lib/ui/btn"

	const title = "shorter | world's first domain shortener"
	const desc = "a domain shortener tool"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
	<meta content={desc} name="description" />
</svelte:head>

<Wave />

<div class="pt-24">
	<div class="flex flex-col items-center justify-center gap-6 lg:gap-10">
		<h1
			class="text-balance text-center font-aeonik text-5xl font-light leading-[1.1] text-white sm:text-7xl sm:leading-none xl:text-[90px]"
		>
			world's first <br class="hidden lg:flex" /> domain shortener
		</h1>
		<h3 class="max-w-xl text-center text-base font-medium text-neutral-300 lg:text-lg">
			searches through thousands of tlds to shorten your domain, like linktree.com -->
			linktr.ee
		</h3>
		<div class="flex items-center gap-x-2">
			<Btn class="group gap-x-[5px] duration-200 ease-in-out" intent="gradient">
				try for free
				<Arrow />
			</Btn>
			<Btn
				class="duration-200 ease-in-out"
				href="https://github.com/vloe/shorter"
				intent="secondary"
				rel="noopener noreferrer"
				target="_blank"
			>
				star on github
			</Btn>
		</div>
	</div>
</div>
