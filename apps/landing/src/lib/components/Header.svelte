<script lang="ts">
	import Lockup from "@sh/assets/svgs/lockup.svg"
	import { Btn } from "@sh/ui/src/lib/components/btn"
	import * as DropdownMenu from "@sh/ui/src/lib/components/dropdown-menu"
	import Arrow from "@sh/ui/src/lib/icons/Arrow.svelte"
	import Menu from "@sh/ui/src/lib/icons/Menu.svelte"
</script>

<header class="flex h-16 items-center justify-between">
	<a href="/">
		<img alt="lockup" class="h-5" src={Lockup} />
	</a>
	<div class="hidden lg:flex">
		<div class="flex items-center gap-x-2">
			<Btn
				class="group h-8 gap-x-[5px] rounded-lg transition-all duration-200 ease-in-out hover:bg-primary hover:brightness-[85%]"
				href="/register"
			>
				start now
				<Arrow />
			</Btn>
			<Btn
				class="!no-underline transition-all duration-200 ease-in-out hover:text-primary hover:brightness-[85%]"
				href="https://github.com/vloe/shorter"
				intent="link"
				rel="noopener noreferrer"
				target="_blank"
			>
				star on github
			</Btn>
		</div>
	</div>
	<div class="flex lg:hidden">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger>
				<Menu />
			</DropdownMenu.Trigger>
			<DropdownMenu.Content class="w-44">
				<DropdownMenu.Item href="/register">start now</DropdownMenu.Item>
				<DropdownMenu.Item
					href="https://github.com/vloe/shorter"
					rel="noopener noreferrer"
					target="_blank"
				>
					repo
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>
