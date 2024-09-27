<script lang="ts">
	import { Lockup } from "$lib/components/icons/lockup"
	import { Logomark } from "$lib/components/icons/logomark"
	import { Menu } from "$lib/components/icons/menu"
	import { ArrowBtn } from "$lib/components/ui/arrow-btn"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
</script>

<header class="flex h-16 items-center justify-between lg:h-[72px]">
	<a href="/">
		<Lockup class="hidden lg:flex" />
		<Logomark class="flex lg:hidden" />
	</a>
	<nav class="hidden items-center gap-x-3 lg:flex">
		<ArrowBtn class="h-8 rounded-lg duration-200 ease-in-out" href="/search">
			start now
		</ArrowBtn>
	</nav>
	<div class="flex lg:hidden">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger><Menu /></DropdownMenu.Trigger>
			<DropdownMenu.Content class="w-52 rounded-lg">
				<DropdownMenu.Item class="rounded-md" href="/search">start now</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>
