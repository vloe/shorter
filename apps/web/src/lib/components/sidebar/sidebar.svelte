<script lang="ts">
	import { page } from "$app/stores"
	import { Logomark } from "@sh/ui/src/lib/components/icons/logomark"
	import * as Avatar from "@sh/ui/src/lib/components/ui/avatar"

	import { navLinks } from "./index"
</script>

<asize class="flex h-auto w-24 flex-col items-center justify-between border-r py-6">
	<a href="/">
		<Logomark class="size-10" />
	</a>
	<div class="flex w-full flex-col space-y-0.5">
		{#each navLinks as { href, icon, name }}
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
