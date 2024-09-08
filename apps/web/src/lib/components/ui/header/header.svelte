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
		<div class="hidden items-center gap-x-1.5 lg:flex">
			<ArrowBtn class="h-8 rounded-lg duration-200 ease-in-out" href="/register">
				start now
			</ArrowBtn>
			<Btn class="duration-200 ease-in-out" href="/login" intent="minimal">login</Btn>
		</div>
		<div class="flex lg:hidden">
			<DropdownMenu.Root>
				<DropdownMenu.Trigger><Menu /></DropdownMenu.Trigger>
				<DropdownMenu.Content class="w-48">
					<DropdownMenu.Item href="/register">start now</DropdownMenu.Item>
					<DropdownMenu.Item href="/login">login</DropdownMenu.Item>
				</DropdownMenu.Content>
			</DropdownMenu.Root>
		</div>
		<hr
			class="absolute bottom-0 h-px w-full border-0 bg-gradient-to-r from-transparent via-white to-transparent opacity-10"
		/>
	</div>
</header>
