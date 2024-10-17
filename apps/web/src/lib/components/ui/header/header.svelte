<script lang="ts">
	import type { FeedbackPayload, FeedbackRes } from "$lib/types/bindings"

	import { Github } from "$lib/components/icons/github"
	import { Logomark } from "$lib/components/icons/logomark"
	import { Menu } from "$lib/components/icons/menu"
	import { Search } from "$lib/components/icons/search"
	import { ArrowBtn } from "$lib/components/ui/arrow-btn"
	import { Btn } from "$lib/components/ui/btn"
	import * as Card from "$lib/components/ui/card"	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
	import * as Popover from "$lib/components/ui/popover"
	import { Textarea } from "$lib/components/ui/textarea"
	import { feedback } from "$lib/queries/feedback"
	import { createMutation } from "@tanstack/svelte-query"

	let popoverOpen = $state(false)

	let payload = $state<FeedbackPayload>({
		msg: "",
	})

	let mutation = createMutation<FeedbackRes, Error>(() => ({
		enabled: false,
		mutationFn: () => feedback(payload),
		mutationKey: ["feedback", payload],
		onSuccess: () => {
			popoverOpen = false
		},
		retry: false,
	}))
</script>

<header class="flex h-20 items-center justify-between">
	<a href="/">
		<Logomark />
	</a>
	<nav class="absolute left-1/2 hidden -translate-x-1/2 items-center gap-6 md:flex">
		<a
			class="text-white/70 hover:text-white"
			href="https://github.com/vloe/shorter"
			rel="noopener noreferrer"
			target="_blank"
		>
			repo
		</a>
		<Popover.Root bind:open={popoverOpen}>
			<Popover.Trigger class="text-white/60 hover:text-white">feedback</Popover.Trigger>
			<Popover.Content class="border-0 p-0">
				<Card.Root>
					<Card.Header>
						<Card.Title>feedback</Card.Title>
						<Card.Description>
							include your email if you want a response
						</Card.Description>
					</Card.Header>
					<Card.Content>
						<Textarea bind:value={payload.msg} placeholder="write a message..." />
						{#if mutation.isError}
							<p class="text-sm text-red-500">{mutation.error.message}</p>
						{/if}
					</Card.Content>
					<Card.Footer class="flex justify-between">
						<Btn intent="outline" onclick={() => (popoverOpen = false)} size="sm">
							cancel
						</Btn>
						<Btn onclick={() => mutation.mutate()} size="sm">send</Btn>
					</Card.Footer>
				</Card.Root>
			</Popover.Content>
		</Popover.Root>
	</nav>
	<ArrowBtn class="hidden rounded-full duration-200 ease-in-out md:flex" href="/search">
		start now
	</ArrowBtn>
	<div class="flex md:hidden">
		<DropdownMenu.Root closeOnItemClick={false}>
			<DropdownMenu.Trigger><Menu /></DropdownMenu.Trigger>
			<DropdownMenu.Content class="w-52">
				<DropdownMenu.Item href="/search">
					<Search class="mr-2.5" />
					start now
				</DropdownMenu.Item>
				<DropdownMenu.Item
					href="https://github.com/vloe/shorter"
					rel="noopener noreferrer"
					target="_blank"
				>
					<Github class="mr-2.5" />
					repo
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>
