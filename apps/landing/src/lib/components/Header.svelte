<script lang="ts">
	import { dev } from "$app/environment"
	import LockupWhite from "@sh/assets/svgs/lockup-white.svg"
	import LogomarkWhite from "@sh/assets/svgs/logomark-white.svg"
	import { ArrowBtn } from "@sh/ui/src/lib/components/arrow-btn"
	import { Btn } from "@sh/ui/src/lib/components/btn"
	import * as DropdownMenu from "@sh/ui/src/lib/components/dropdown-menu"
	import Menu from "@sh/ui/src/lib/icons/Menu.svelte"
	import { webUrl } from "@sh/utils/src/urls"
</script>

<header class="flex h-16 items-center justify-between lg:h-20">
	<a href="/">
		<img alt="lockup" class="hidden h-7 lg:flex" src={LockupWhite} />
		<img alt="lockup" class="flex h-7 lg:hidden" src={LogomarkWhite} />
	</a>
	<div class="hidden items-center gap-x-1.5 lg:flex">
		<ArrowBtn
			class="h-8 rounded-lg duration-200 ease-in-out hover:bg-primary/85"
			href={`${webUrl(dev)}/register`}
		>
			start now
		</ArrowBtn>
		<Btn
			class="!no-underline duration-200 ease-in-out hover:text-primary/85"
			href={`${webUrl(dev)}/login`}
			intent="link"
		>
			login
		</Btn>
	</div>
	<div class="flex lg:hidden">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				<Menu class="size-7" />
			</DropdownMenu.Trigger>
			<DropdownMenu.Content class="w-52">
				<DropdownMenu.Item href={`${webUrl(dev)}/register`}>start now</DropdownMenu.Item>
				<DropdownMenu.Item href={`${webUrl(dev)}/login`}>login</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>
