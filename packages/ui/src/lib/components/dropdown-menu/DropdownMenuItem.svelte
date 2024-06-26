<script lang="ts">
	import { DropdownMenu } from "bits-ui"
	import type { Snippet } from "svelte"
	import type { HTMLAnchorAttributes } from "svelte/elements"
	import { cn } from "../../utils/cn"

	interface $Props extends HTMLAnchorAttributes {
		children: Snippet
		class?: string
	}

	let { children, class: className, ...props }: $Props = $props()
</script>

<DropdownMenu.Item
	{...props}
	class={cn(
		"rounded-button flex h-10 select-none items-center py-3 pl-3 pr-1.5 text-sm font-medium !ring-0 !ring-transparent data-[highlighted]:bg-muted",
		className,
	)}
>
	{@render children()}
</DropdownMenu.Item>
