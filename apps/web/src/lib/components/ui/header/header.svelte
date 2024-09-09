<script lang="ts">
	import { Lockup } from "$lib/components/icons/lockup"
	import { Logomark } from "$lib/components/icons/logomark"
	import { Menu } from "$lib/components/icons/menu"
	import { ArrowBtn } from "$lib/components/ui/arrow-btn"
	import { Btn } from "$lib/components/ui/btn"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
</script>

<header class="fixed z-50 h-14 w-full backdrop-blur-lg">
	<div class="container h-full">
		<div class="flex h-full items-center justify-between">
			<a href="/">
				<Lockup class="hidden lg:flex" />
				<Logomark class="flex lg:hidden" />
			</a>
			<nav class="hidden items-center gap-x-1.5 lg:flex">
				<ArrowBtn class="duration-200 ease-in-out" href="/search">start now</ArrowBtn>
				<Btn class="duration-200 ease-in-out" disabled intent="minimal">login</Btn>
			</nav>
			<div class="flex lg:hidden">
				<DropdownMenu.Root>
					<DropdownMenu.Trigger><Menu /></DropdownMenu.Trigger>
					<DropdownMenu.Content class="w-48">
						<DropdownMenu.Item href="/search">start now</DropdownMenu.Item>
						<DropdownMenu.Item disabled>login</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</div>
		</div>
	</div>
	<hr
		class="absolute bottom-0 h-px w-full border-0 opacity-10"
		style="background-image:linear-gradient(270deg, rgba(255, 255, 255, 0.00) 0%, #FFF 52.07%, rgba(255, 255, 255, 0.00) 100%)"
	/>
</header>
