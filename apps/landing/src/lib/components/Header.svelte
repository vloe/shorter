<script lang="ts">
<<<<<<< HEAD
	import Lockup from "$lib/assets/svgs/lockup.svg"
	import Logomark from "$lib/assets/svgs/logomark.svg"
=======
	import Logomark from "$lib/assets/svgs/logomark.svg"
	import Lockup from "$lib/assets/svgs/lockup.svg"
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
	import Arrow from "$lib/icons/Arrow.svelte"
	import Menu from "$lib/icons/Menu.svelte"
	import { Btn } from "$lib/ui/btn"
	import {
		DropdownMenu,
<<<<<<< HEAD
		DropdownMenuContent,
		DropdownMenuItem,
		DropdownMenuTrigger,
=======
		DropdownMenuTrigger,
		DropdownMenuContent,
		DropdownMenuItem,
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
	} from "$lib/ui/dropdown-menu"
</script>

<header class="flex h-[72px] flex-row items-center justify-between">
<<<<<<< HEAD
	<a class="rounded-md bg-black px-2 py-1" href="/">
		<img alt="logomark" class="flex h-7 lg:hidden" src={Logomark} />
		<img alt="lockup" class="hidden h-7 lg:flex" src={Lockup} />
=======
	<a href="/" class="rounded-md bg-black px-2 py-1">
		<img class="flex h-7 lg:hidden" src={Logomark} alt="logomark" />
		<img class="hidden h-7 lg:flex" src={Lockup} alt="lockup" />
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
	</a>
	<div class="hidden items-center gap-x-2 lg:flex">
		<Btn class="group gap-x-[5px] mix-blend-lighten duration-200 ease-in-out hover:bg-white">
			try now
			<Arrow />
		</Btn>
		<Btn
<<<<<<< HEAD
			class="duration-200 ease-in-out"
			href="https://github.com/vloe/shorter"
			intent="secondary"
			rel="noopener noreferrer"
			target="_blank"
=======
			intent="secondary"
			href="https://github.com/vloe/shorter"
			target="_blank"
			rel="noopener noreferrer"
			class="duration-200 ease-in-out"
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
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
<<<<<<< HEAD
					rel="noopener noreferrer"
					target="_blank"
=======
					target="_blank"
					rel="noopener noreferrer"
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
				>
					repo
				</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	</div>
</header>
