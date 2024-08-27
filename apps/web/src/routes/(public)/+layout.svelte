<script lang="ts">
	import type { Snippet } from "svelte"

	import Lockup from "@sh/assets/svgs/lockup.svg"
	import * as Card from "@sh/ui/src/lib/components/card"

	type $Props = {
		children: Snippet
	}

	let { children }: $Props = $props()
</script>

<div class="flex min-h-screen items-center justify-center scroll-smooth px-6">
	<div class="w-full max-w-lg">
		<div class="mb-8 ml-6">
			<img alt="lockup" class="h-5" src={Lockup} />
		</div>
		<Card.Root class="flex min-h-64 w-full max-w-lg flex-col gap-y-3 py-6">
			{@render children()}
		</Card.Root>
	</div>
</div>
