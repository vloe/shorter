<script lang="ts">
	import { Lockup } from "$lib/components/icons/lockup"
	import { Logomark } from "$lib/components/icons/logomark"
	import { Menu } from "$lib/components/icons/menu"
	import { ArrowBtn } from "$lib/components/ui/arrow-btn"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
</script>

<div class="container">
	<header class="flex h-16 items-center justify-between">
		<a href="/">
			<Lockup class="hidden lg:flex" />
			<Logomark class="flex lg:hidden" />
		</a>
		<ArrowBtn class="hidden duration-200 ease-in-out lg:flex" href="/search">
			start now
		</ArrowBtn>
		<div class="flex lg:hidden">
			<DropdownMenu.Root>
				<DropdownMenu.Trigger><Menu /></DropdownMenu.Trigger>
				<DropdownMenu.Content class="w-48">
					<DropdownMenu.Item href="/search">start now</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		</div>
	</header>
</div>
