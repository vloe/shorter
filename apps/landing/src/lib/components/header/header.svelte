<script lang="ts">
	import { webUrl } from "$lib/utils/webUrl"
	import { Lockup } from "@sh/ui/src/lib/components/icons/lockup"
	import { Logomark } from "@sh/ui/src/lib/components/icons/logomark"
	import { Menu } from "@sh/ui/src/lib/components/icons/menu"
	import { ArrowButton } from "@sh/ui/src/lib/components/ui/arrow-button"
	import { Button } from "@sh/ui/src/lib/components/ui/button"
	import * as DropdownMenu from "@sh/ui/src/lib/components/ui/dropdown-menu"
</script>

<header class="flex h-16 items-center justify-between lg:h-20">
	<a href="/">
		<Lockup class="hidden lg:flex" />
		<Logomark class="flex lg:hidden" />
	</a>
	<div class="hidden items-center gap-x-1.5 lg:flex">
		<ArrowButton
			class="h-8 rounded-lg duration-200 ease-in-out hover:bg-primary/85"
			href={`${webUrl}/`}
		>
			start now
		</ArrowButton>
		<Button
			class="!no-underline duration-200 ease-in-out hover:text-primary/85"
			disabled
			variant="link"
		>
			login
		</Button>
	</div>
	<div class="flex lg:hidden">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				<Menu />
			</DropdownMenu.Trigger>
			<DropdownMenu.Content class="w-44">
				<DropdownMenu.Item href={`${webUrl}/`}>start now</DropdownMenu.Item>
				<DropdownMenu.Item disabled href={`${webUrl}/login`}>login</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>
