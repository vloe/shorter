<script lang="ts">
	import { Feedback } from "$lib/components/icons/feedback"
	import { Lockup } from "$lib/components/icons/lockup"
	import { Logomark } from "$lib/components/icons/logomark"
	import { Btn } from "$lib/components/ui/btn"
	import * as Popover from "$lib/components/ui/popover"
</script>

<header class="flex h-20 items-center justify-between">
	<a href="/">
		<Lockup class="hidden lg:flex" />
		<Logomark class="flex lg:hidden" />
	</a>
	<Popover.Root>
		<Popover.Trigger asChild let:builder>
			<Btn builders={[builder]} class="gap-x-1 duration-200" intent="default">
				<Feedback />
				feedback
			</Btn>
		</Popover.Trigger>
		<Popover.Content>test</Popover.Content>
	</Popover.Root>
</header>
