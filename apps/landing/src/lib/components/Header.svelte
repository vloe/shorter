<script lang="ts">
	import Logomark from "$lib/assets/svgs/logomark.svg"
	import Lockup from "$lib/assets/svgs/lockup.svg"
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
	<a href="/" class="rounded-md bg-black px-2 py-1">
		<img class="flex h-7 lg:hidden" src={Logomark} alt="logomark" />
		<img class="hidden h-7 lg:flex" src={Lockup} alt="lockup" />
	</a>
	<div class="hidden items-center gap-x-2 lg:flex">
		<Btn class="group gap-x-[5px] mix-blend-lighten duration-200 ease-in-out hover:bg-white">
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
			<DropdownMenuTrigger asChild let:builder>
				<Btn builders={[builder]} intent="ghost" size="icon">
					<Menu class="text-white" />
				</Btn>
			</DropdownMenuTrigger>
			<DropdownMenuContent class="w-40">
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
