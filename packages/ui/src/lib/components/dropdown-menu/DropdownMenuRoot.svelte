<script lang="ts">
	import { DropdownMenu } from "bits-ui"
	import type { Snippet } from "svelte"

	interface $Props {
		children: Snippet
	}

	let { children }: $Props = $props()
</script>

<DropdownMenu.Root>
	{@render children()}
</DropdownMenu.Root>
