<script lang="ts">
	import LockupWhite from "@sh/assets/svgs/lockup-white.svg"
	import LogomarkWhite from "@sh/assets/svgs/logomark-white.svg"
	import { Btn } from "@sh/ui/src/lib/components/btn"
	import * as DropdownMenu from "@sh/ui/src/lib/components/dropdown-menu"
	import Arrow from "@sh/ui/src/lib/icons/Arrow.svelte"
	import Menu from "@sh/ui/src/lib/icons/Menu.svelte"
</script>

<header class="flex h-20 items-center justify-between">
	<a href="/">
		<img alt="lockup" class="hidden h-7 lg:flex" src={LockupWhite} />
		<img alt="lockup" class="flex h-7 lg:hidden" src={LogomarkWhite} />
	</a>
	<div class="hidden items-center gap-x-1.5 lg:flex">
		<Btn
			class="group h-8 gap-x-[5px] rounded-lg transition-all duration-200 ease-in-out hover:bg-primary/85"
			disabled
		>
			start now
			<Arrow />
		</Btn>
		<Btn
			class="disabled !no-underline transition-all duration-200 ease-in-out hover:text-white/85"
			disabled
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
				<DropdownMenu.Item disabled>start now</DropdownMenu.Item>
				<DropdownMenu.Item disabled>login</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>
