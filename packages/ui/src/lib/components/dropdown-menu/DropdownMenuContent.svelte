<script lang="ts">
	import { DropdownMenu } from "bits-ui"
	import type { Snippet } from "svelte"
	import { cn } from "../../utils/cn"
	import { flyAndScale } from "../../utils/flyAndScale"

	interface $Props {
		children: Snippet
		class?: string
	}

	let { children, class: className }: $Props = $props()
</script>

<DropdownMenu.Content
	transition={flyAndScale}
	sideOffset={8}
	class={cn(
		"w-full max-w-[229px] rounded-xl border border-muted bg-background px-1 py-1.5 shadow-popover",
		className,
	)}
>
	{@render children()}
</DropdownMenu.Content>
