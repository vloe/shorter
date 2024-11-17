<script lang="ts">
	import { Github } from "$lib/components/icons/github"
	import { Lockup } from "$lib/components/icons/lockup"
	import { Logomark } from "$lib/components/icons/logomark"
	import { Menu } from "$lib/components/icons/menu"
	import { Search } from "$lib/components/icons/search"
	import { ArrowBtn } from "$lib/components/ui/arrow-btn"
	import { Btn } from "$lib/components/ui/btn"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
</script>

<header class="flex h-[72px] items-center justify-between">
	<a href="/">
		<Lockup class="hidden lg:flex" />
		<Logomark class="flex lg:hidden" />
	</a>
	<div class="hidden items-center gap-2 lg:flex">
		<ArrowBtn class="duration-300 ease-in-out" href="/search">Start now</ArrowBtn>
		<Btn
			class="duration-300 ease-in-out"
			href="https://github.com/vloe/shorter"
			rel="noopener noreferrer"
			target="_blank"
			variant="minimal"
		>
			Repo
		</Btn>
	</div>
	<div class="flex lg:hidden">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger><Menu /></DropdownMenu.Trigger>
			<DropdownMenu.Content class="w-52">
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a href="/search" {...props}>
							<Search />
							Start now
						</a>
					{/snippet}
				</DropdownMenu.Item>
				<DropdownMenu.Item>
					{#snippet child({ props })}
						<a
							href="https://github.com/vloe/shorter"
							rel="noopener noreferrer"
							target="_blank"
							{...props}
						>
							<Github />
							Repo
						</a>
					{/snippet}
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>
