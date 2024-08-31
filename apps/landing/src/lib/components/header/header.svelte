<script lang="ts">
	import { webUrl } from "$lib/utils/webUrl"
	import Lockup from "@sh/assets/svgs/lockup.svg"
	import Logomark from "@sh/assets/svgs/logomark.svg"
	import Menu from "@sh/assets/svgs/menu.svg"
	import { ArrowButton } from "@sh/ui/src/lib/components/ui/arrow-button"
	import { Button } from "@sh/ui/src/lib/components/ui/button"
	import * as DropdownMenu from "@sh/ui/src/lib/components/ui/dropdown-menu"
</script>

<header class="flex h-16 items-center justify-between lg:h-20">
	<a href="/">
		<img alt="lockup" class="hidden h-7 lg:flex" src={Lockup} />
		<img alt="logomark" class="flex h-7 lg:hidden" src={Logomark} />
	</a>
	<div class="hidden items-center gap-x-1.5 lg:flex">
		<ArrowButton
			class="h-8 rounded-lg duration-200 ease-in-out hover:bg-primary/85"
			href={`${webUrl}/register`}
		>
			start now
		</ArrowButton>
		<Button
			class="!no-underline duration-200 ease-in-out hover:text-primary/85"
			href={`${webUrl}/login`}
			variant="link"
		>
			login
		</Button>
	</div>
	<div class="flex lg:hidden">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				<img alt="menu" class="size-6" src={Menu} />
			</DropdownMenu.Trigger>
			<DropdownMenu.Content class="w-44">
				<DropdownMenu.Item href={`${webUrl}/register`}>start now</DropdownMenu.Item>
				<DropdownMenu.Item href={`${webUrl}/login`}>login</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>
