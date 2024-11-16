<script lang="ts">
	import { Github } from "$lib/components/icons/github"
	import { Logomark } from "$lib/components/icons/logomark"
	import { Menu } from "$lib/components/icons/menu"
	import { Search } from "$lib/components/icons/search"
	import { ArrowBtn } from "$lib/components/ui/arrow-btn"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
</script>

<header class="flex h-20 items-center justify-between">
	<a href="/">
		<Logomark />
	</a>
	<nav class="absolute left-1/2 hidden -translate-x-1/2 items-center gap-6 md:flex">
		<a
			class="text-white/70 hover:text-white"
			href="https://github.com/vloe/shorter"
			rel="noopener noreferrer"
			target="_blank"
		>
			repo
		</a>
	</nav>
	<ArrowBtn class="hidden rounded-full duration-200 ease-in-out md:flex" href="/search">
		start now
	</ArrowBtn>
	<div class="flex md:hidden">
		<DropdownMenu.Root closeOnItemClick={false}>
			<DropdownMenu.Trigger><Menu /></DropdownMenu.Trigger>
			<DropdownMenu.Content class="w-52">
				<DropdownMenu.Item href="/search">
					<Search class="mr-2.5" />
					start now
				</DropdownMenu.Item>
				<DropdownMenu.Item
					href="https://github.com/vloe/shorter"
					rel="noopener noreferrer"
					target="_blank"
				>
					<Github class="mr-2.5" />
					repo
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>
