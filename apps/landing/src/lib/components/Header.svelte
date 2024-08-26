<script lang="ts">
	import { dev } from "$app/environment"
	import Lockup from "@sh/assets/svgs/lockup.svg"
	import { Btn } from "@sh/ui/src/lib/components/btn"
	import * as DropdownMenu from "@sh/ui/src/lib/components/dropdown-menu"
	import Arrow from "@sh/ui/src/lib/icons/Arrow.svelte"
	import Menu from "@sh/ui/src/lib/icons/Menu.svelte"
	import { webUrl } from "@sh/utils/src/urls"
</script>

<header class="flex h-16 items-center justify-between">
	<a href="/">
		<img alt="lockup" class="h-5" src={Lockup} />
	</a>
	<div class="hidden lg:flex">
		<div class="flex items-center gap-x-1.5">
			<Btn
				class="group h-8 gap-x-[5px] rounded-lg transition-all duration-200 ease-in-out hover:bg-primary/85"
				href={`${webUrl(dev)}/register`}
			>
				start now
				<Arrow />
			</Btn>
			<Btn
				class="disabled !no-underline transition-all duration-200 ease-in-out hover:text-white/85"
				href={`${webUrl(dev)}/login`}
				intent="link"
			>
				login
			</Btn>
		</div>
	</div>
	<div class="flex lg:hidden">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				<Menu />
			</DropdownMenu.Trigger>
			<DropdownMenu.Content class="w-44">
				<DropdownMenu.Item href={`${webUrl(dev)}/register`}>start now</DropdownMenu.Item>
				<DropdownMenu.Item href={`${webUrl(dev)}/login`}>login</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>
