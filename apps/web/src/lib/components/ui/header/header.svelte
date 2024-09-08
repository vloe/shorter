<script lang="ts">
	import { Lockup } from "$lib/components/icons/lockup"
	import { Logomark } from "$lib/components/icons/logomark"
	import { Menu } from "$lib/components/icons/menu"
	import { ArrowBtn } from "$lib/components/ui/arrow-btn"
	import { Btn } from "$lib/components/ui/btn"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
</script>

<header class="sticky top-0 z-50 w-full backdrop-blur-lg">
	<div class="relative flex h-16 items-center justify-between lg:h-[72px]">
		<a href="/">
			<Lockup class="hidden lg:flex" />
			<Logomark class="flex lg:hidden" />
		</a>
		<nav class="hidden lg:flex">
			<ul class="flex items-center gap-x-4">
				<li><Btn disabled intent="minimal">pricing</Btn></li>
				<li><Btn disabled intent="minimal">bookmarks</Btn></li>
				<li><Btn disabled intent="minimal">links</Btn></li>
			</ul>
		</nav>
		<nav class="hidden items-center gap-x-1.5 lg:flex">
			<ArrowBtn class="h-8 rounded-lg duration-200 ease-in-out" href="/search">
				start now
			</ArrowBtn>
			<Btn class="duration-200 ease-in-out" disabled intent="minimal">login</Btn>
		</nav>
		<div class="flex lg:hidden">
			<DropdownMenu.Root>
				<DropdownMenu.Trigger><Menu /></DropdownMenu.Trigger>
				<DropdownMenu.Content class="w-48">
					<DropdownMenu.Item href="/search">start now</DropdownMenu.Item>
					<DropdownMenu.Item disabled>login</DropdownMenu.Item>
					<DropdownMenu.Separator />
					<DropdownMenu.Item disabled>pricing</DropdownMenu.Item>
					<DropdownMenu.Item disabled>bookmarks</DropdownMenu.Item>
					<DropdownMenu.Item disabled>links</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		</div>
		<hr
			class="absolute bottom-0 h-px w-full border-0 bg-gradient-to-r from-transparent via-white to-transparent opacity-10"
		/>
	</div>
</header>
