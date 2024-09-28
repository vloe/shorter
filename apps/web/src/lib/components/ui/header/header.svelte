<script lang="ts">
	import type { FeedbackPayload, FeedbackRes } from "$lib/utils/bindings"

	import { Feedback } from "$lib/components/icons/feedback"
	import { Lockup } from "$lib/components/icons/lockup"
	import { Logomark } from "$lib/components/icons/logomark"
	import { Menu } from "$lib/components/icons/menu"
	import { ArrowBtn } from "$lib/components/ui/arrow-btn"
	import { Button } from "$lib/components/ui/button"
	import * as Card from "$lib/components/ui/card"
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu"
	import * as Popover from "$lib/components/ui/popover"
	import { Textarea } from "$lib/components/ui/textarea"
	import { feedback } from "$lib/queries/feedback"
	import { createMutation } from "@tanstack/svelte-query"

	let popoverOpen = $state(false)

	let feedbackPayload = $state<FeedbackPayload>({
		msg: "",
	})

	let feedbackQuery = createMutation<FeedbackRes, Error>(() => ({
		enabled: false,
		mutationFn: () => feedback(feedbackPayload),
		mutationKey: ["feedback", feedbackPayload],
		onSuccess: () => {
			popoverOpen = false
		},
		retry: false,
	}))
</script>

<header class="flex h-16 items-center justify-between lg:h-[72px]">
	<a href="/">
		<Lockup class="hidden lg:flex" />
		<Logomark class="flex lg:hidden" />
	</a>
	<nav class="hidden items-center gap-x-3 lg:flex">
		<Popover.Root bind:open={popoverOpen}>
			<Popover.Trigger asChild let:builder>
				<Button builders={[builder]} class="h-8 rounded-lg" size="icon" variant="outline">
					<Feedback />
				</Button>
			</Popover.Trigger>
			<Popover.Content class="rounded-lg p-0">
				<Card.Root class="border-none p-0">
					<Card.Header>
						<Card.Title>feedback</Card.Title>
						<Card.Description>drop your email if you want a response</Card.Description>
					</Card.Header>
					<Card.Content class="py-4">
						<Textarea bind:value={feedbackPayload.msg} class="focus-visible:ring-0" />
						{#if feedbackQuery.isError}
							<p class="text-sm text-red-500">{feedbackQuery.error.message}</p>
						{/if}
					</Card.Content>
					<Card.Footer class="flex justify-between">
						<Button
							class="h-7 rounded-lg"
							onclick={() => (popoverOpen = false)}
							variant="outline"
						>
							cancel
						</Button>
						<Button class="h-7 rounded-lg" onclick={() => feedbackQuery.mutate()}>
							send
						</Button>
					</Card.Footer>
				</Card.Root>
			</Popover.Content>
		</Popover.Root>

		<ArrowBtn class="h-8 rounded-lg duration-200 ease-in-out" href="/search">
			start now
		</ArrowBtn>
	</nav>
	<div class="flex lg:hidden">
		<DropdownMenu.Root>
			<DropdownMenu.Trigger><Menu /></DropdownMenu.Trigger>
			<DropdownMenu.Content class="w-52 rounded-lg">
				<DropdownMenu.Item class="rounded-md" href="/search">start now</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>
