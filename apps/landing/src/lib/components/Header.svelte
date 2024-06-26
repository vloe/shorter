<script lang="ts">
	import Button from "@sh/ui/src/lib/components/Button.svelte"
	import Link from "@sh/ui/src/lib/components/Link.svelte"
	import Menu from "@sh/ui/src/lib/icons/Menu.svelte"
	import DropdownMenuRoot from "@sh/ui/src/lib/components/dropdown-menu/DropdownMenuRoot.svelte"
	import DropdownMenuTrigger from "@sh/ui/src/lib/components/dropdown-menu/DropdownMenuTrigger.svelte"
	import DropdownMenuItem from "@sh/ui/src/lib/components/dropdown-menu/DropdownMenuItem.svelte"
	import DropdownMenuContent from "@sh/ui/src/lib/components/dropdown-menu/DropdownMenuContent.svelte"
</script>

<header class="flex h-16 items-center justify-between">
	<a href="/">
		<h2 class="font-lumiera text-3xl">shorter</h2>
	</a>
	<div class="hidden lg:flex">
		<div class="flex items-center space-x-2">
			<Button class="rounded-xl">start now</Button>
			<Link
				href="https://github.com/vloe/shorter"
				target="_blank"
				rel="noreferrer noopener"
				class="!no-underline hover:text-primary/75"
			>
				star on github
			</Link>
		</div>
	</div>
	<div class="flex lg:hidden">
		<DropdownMenuRoot>
			<DropdownMenuTrigger>
				<Button variant="ghost" size="icon">
					<Menu />
				</Button>
			</DropdownMenuTrigger>
			<DropdownMenuContent class="w-36">
				<DropdownMenuItem>start now</DropdownMenuItem>
				<DropdownMenuItem
					href="https://github.com/vloe/shorter"
					target="_blank"
					rel="noreferrer noopener"
				>
					repo
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenuRoot>
	</div>
</header>
