<script lang="ts">
	import { Feedback } from "$lib/components/icons/feedback"
	import { Lockup } from "$lib/components/icons/lockup"
	import { Logomark } from "$lib/components/icons/logomark"
	import { Search } from "$lib/components/icons/search"
	import { Btn } from "$lib/components/ui/btn"
	import * as Popover from "$lib/components/ui/popover"

	const title = "shorter | world's first domain shortener"
	const desc = "a domain shortener tool"
</script>

<svelte:head>
	<title>{title}</title>
	<meta content={title} name="title" />
	<meta content={desc} name="description" />
</svelte:head>

<header class="flex h-20 items-center justify-between">
	<a href="/">
		<Lockup class="hidden lg:flex" />
		<Logomark class="flex lg:hidden" />
	</a>
	<Popover.Root>
		<Popover.Trigger asChild let:builder>
			<Btn builders={[builder]} class="gap-x-[5px] duration-200" intent="default">
				<Feedback />
				feedback
			</Btn>
		</Popover.Trigger>
		<Popover.Content>test</Popover.Content>
	</Popover.Root>
</header>

<div class="pt-28 lg:pt-40">
	<div class="flex flex-col items-center gap-6 lg:gap-10">
		<h1
			class="text-center text-5xl font-thin tracking-[-0.9px] sm:text-6xl lg:text-7xl xl:text-[90px]"
		>
			world's first domain <br class="hidden sm:flex" /> shortener
		</h1>
		<h2 class="max-w-md text-center text-white/70 lg:text-lg">
			discover a shorter version of your domain, for example linktree.com -> linktr.ee
		</h2>
		<div
			class="flex h-10 w-full max-w-sm items-center justify-center gap-2 rounded-full border border-white/25 px-3.5 py-1 shadow-sm lg:h-[46px]"
		>
			<Search class="text-white/50 lg:size-4" />
			<input
				autofocus
				class="h-full w-full bg-transparent text-sm outline-none placeholder:text-white/50 lg:text-base"
				placeholder="type any domain..."
			/>
		</div>
	</div>
</div>
