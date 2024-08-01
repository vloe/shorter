<script lang="ts">
	import Logomark from "$lib/assets/svgs/white-logomark.svg"
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

<header class="flex h-16 flex-row items-center justify-between">
	<a href="/">
		<img class="h-7" src={Logomark} alt="logomark" />
	</a>
	<div class="hidden items-center gap-x-2 lg:flex">
		<Btn class="group gap-x-[5px] duration-200 ease-in-out">
			start now
			<Arrow />
		</Btn>
		<Btn intent="secondary" class="duration-200 ease-in-out">login</Btn>
	</div>
	<div class="flex lg:hidden">
		<DropdownMenu>
			<DropdownMenuTrigger asChild let:builder>
				<Btn builders={[builder]} intent="ghost" size="icon"><Menu /></Btn>
			</DropdownMenuTrigger>
			<DropdownMenuContent class="w-48">
				<DropdownMenuItem>test 1</DropdownMenuItem>
				<DropdownMenuItem>test 2</DropdownMenuItem>
				<DropdownMenuItem>test 3</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	</div>
</header>
