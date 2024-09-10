<script lang="ts">
	import { Feedback } from "$lib/components/icons/feedback"
	import { Lockup } from "$lib/components/icons/lockup"
	import { Logomark } from "$lib/components/icons/logomark"
	import { Menu } from "$lib/components/icons/menu"
	import { Btn } from "$lib/components/ui/btn"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
</script>

<header class="fixed z-50 h-14 w-full backdrop-blur-lg">
	<div class="container h-full">
		<div class="flex h-full items-center justify-between">
			<a href="/">
				<Lockup class="hidden lg:flex" />
				<Logomark class="flex lg:hidden" />
			</a>
			<Btn class="hidden gap-x-1 duration-200 ease-in-out lg:flex" disabled>
				<Feedback />
				feedback
			</Btn>
			<div class="flex lg:hidden">
				<DropdownMenu.Root>
					<DropdownMenu.Trigger><Menu /></DropdownMenu.Trigger>
					<DropdownMenu.Content class="w-48">
						<DropdownMenu.Item disabled>feedback</DropdownMenu.Item>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			</div>
		</div>
	</div>
	<hr
		class="absolute bottom-0 h-px w-full border-0 opacity-10"
		style="background-image:linear-gradient(270deg, rgba(255, 255, 255, 0.00) 0%, #FFF 52.07%, rgba(255, 255, 255, 0.00) 100%)"
	/>
</header>
