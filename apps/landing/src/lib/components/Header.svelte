<script lang="ts">
	import * as DropdownMenu from "@sh/ui/src/lib/components/dropdown-menu"
	import { Button } from "@sh/ui/src/lib/components/button"
	import Menu from "$lib/icons/Menu.svelte"
	import { scrollToTable } from "$lib/utils/scrollToTable"
</script>

<header class="flex h-16 items-center justify-between">
	<a href="/">
		<h2 class="font-lumiera text-3xl">shorter</h2>
	</a>
	<div class="hidden lg:flex">
		<div class="flex items-center space-x-2">
			<Button on:click={() => scrollToTable()} class="rounded-xl">start now</Button>
			<Button
				href="https://github.com/vloe/shorter"
				target="_blank"
				rel="noreferrer noopener"
				variant="link"
				class="!no-underline hover:text-primary/75"
			>
				star on github
			</Button>
		</div>
	</div>
	<div class="flex lg:hidden">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger asChild let:builder>
				<Button builders={[builder]} variant="ghost" size="icon">
					<Menu />
				</Button>
			</DropdownMenu.Trigger>
			<DropdownMenu.Content class="w-44">
				<DropdownMenu.Item>start now</DropdownMenu.Item>
				<DropdownMenu.Item
					href="https://github.com/vloe/shorter"
					target="_blank"
					rel="noreferrer noopener"
				>
					repo
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>
