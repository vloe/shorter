<script lang="ts">
	import { page } from "$app/stores"
	import { Bookmarks } from "@sh/ui/src/lib/components/icons/bookmarks"
	import { Links } from "@sh/ui/src/lib/components/icons/links"
	import { Logomark } from "@sh/ui/src/lib/components/icons/logomark"
	import { Search } from "@sh/ui/src/lib/components/icons/search"
	import * as Avatar from "@sh/ui/src/lib/components/ui/avatar"

	const sites = [
		{ href: "/bookmarks", icon: Bookmarks, name: "bookmarks" },
		{ href: "/", icon: Search, name: "search" },
		{ href: "/links", icon: Links, name: "links" },
	]
</script>

<asize class="hidden h-auto w-24 flex-col items-center justify-between border-r py-6 sm:flex">
	<a href="/">
		<Logomark class="size-10" />
	</a>
	<div class="flex w-full flex-col space-y-0.5">
		{#each sites as { href, icon, name }}
			<div class="relative flex h-12 w-full px-0.5">
				<a
					class="flex h-full w-full cursor-default select-none items-center justify-center rounded-md transition-colors duration-200 ease-in-out hover:bg-accent hover:text-accent-foreground"
					{href}
				>
					<svelte:component this={icon} />
				</a>
				{#if $page.url.pathname === href}
					<div class="absolute right-0 h-full w-[3px] rounded-l-sm bg-primary"></div>
				{/if}
			</div>
		{/each}
	</div>
	<Avatar.Root>
		<Avatar.Image alt="@vloe" src="https://github.com/vloe.png" />
		<Avatar.Fallback>v</Avatar.Fallback>
	</Avatar.Root>
</asize>
