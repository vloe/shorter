<script lang="ts">
	import Lockup from "$lib/assets/svgs/lockup.svg"
	import Logomark from "$lib/assets/svgs/logomark.svg"
	import Arrow from "$lib/icons/Arrow.svelte"
	import Menu from "$lib/icons/Menu.svelte"
	import { Btn } from "$lib/ui/btn"
	import {
		DropdownMenu,
		DropdownMenuContent,
		DropdownMenuItem,
		DropdownMenuTrigger,
	} from "$lib/ui/dropdown-menu"
</script>

<header class="flex h-[72px] flex-row items-center justify-between">
	<a class="rounded-md bg-black px-2 py-1" href="/">
		<img alt="logomark" class="flex h-7 lg:hidden" src={Logomark} />
		<img alt="lockup" class="hidden h-7 lg:flex" src={Lockup} />
	</a>
	<div class="hidden items-center gap-x-2 lg:flex">
		<Btn class="group gap-x-[5px] mix-blend-lighten duration-200 ease-in-out hover:bg-white">
			try now
			<Arrow />
		</Btn>
		<Btn
			class="duration-200 ease-in-out"
			href="https://github.com/vloe/shorter"
			intent="secondary"
			rel="noopener noreferrer"
			target="_blank"
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
				<DropdownMenuItem>try now</DropdownMenuItem>
				<DropdownMenuItem
					href="https://github.com/vloe/shorter"
					rel="noopener noreferrer"
					target="_blank"
				>
					repo
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	</div>
</header>
