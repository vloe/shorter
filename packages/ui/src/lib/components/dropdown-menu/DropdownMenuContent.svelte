<script lang="ts">
	import { cn } from "../../utils/cn"
	import { flyAndScale } from "../../utils/flyAndScale"
	import { DropdownMenu } from "bits-ui"
	import type { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"
	import type { Snippet } from "svelte"

	type $Props = DropdownMenuPrimitive.ContentProps & {
		children: Snippet
	}

	let { children, ...props }: $Props = $props()
</script>

<DropdownMenu.Content
	{...props}
	transition={flyAndScale}
	sideOffset={8}
	class={cn(
		"w-full max-w-[229px] rounded-xl border border-muted bg-background px-1 py-1.5 shadow-popover",
		props.class,
	)}
>
	{@render children()}
</DropdownMenu.Content>
