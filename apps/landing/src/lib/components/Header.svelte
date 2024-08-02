<script lang="ts">
	import Logomark from "$lib/assets/svgs/white-logomark.svg"
	import Lockup from "$lib/assets/svgs/white-lockup.svg"
	import Arrow from "$lib/icons/Arrow.svelte"
	import Menu from "$lib/icons/Menu.svelte"
	import { Btn } from "$lib/ui/btn"
	import {
		DropdownMenu,
		DropdownMenuTrigger,
		DropdownMenuContent,
		DropdownMenuItem,
	} from "$lib/ui/dropdown-menu"
</script>

<header class="flex h-[72px] flex-row items-center justify-between">
	<a href="/">
		<img class="flex h-7 lg:hidden" src={Logomark} alt="logomark" />
		<img class="hidden h-7 lg:flex" src={Lockup} alt="lockup" />
	</a>
	<div class="hidden items-center gap-x-2 lg:flex">
		<Btn class="group gap-x-[5px] duration-200 ease-in-out">
			start now
			<Arrow />
		</Btn>
		<Btn
			intent="secondary"
			href="https://github.com/vloe/shorter"
			target="_blank"
			rel="noopener noreferrer"
			class="duration-200 ease-in-out"
		>
			star on github
		</Btn>
	</div>
	<div class="flex lg:hidden">
		<DropdownMenu>
			<DropdownMenuTrigger>
				<Menu class="size-7 text-white" />
			</DropdownMenuTrigger>
			<DropdownMenuContent class="w-48">
				<DropdownMenuItem>start now</DropdownMenuItem>
				<DropdownMenuItem
					href="https://github.com/vloe/shorter"
					target="_blank"
					rel="noopener noreferrer"
				>
					repo
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	</div>
</header>
