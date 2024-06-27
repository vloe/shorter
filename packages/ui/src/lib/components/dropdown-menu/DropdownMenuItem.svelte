<script lang="ts">
	import { cn } from "../../utils/cn"
	import { DropdownMenu } from "bits-ui"
	import type { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"
	import type { Snippet } from "svelte"

	type $Props = DropdownMenuPrimitive.ItemProps & {
		children: Snippet
	}

	let { children, ...props }: $Props = $props()
</script>

<DropdownMenu.Item
	{...props}
	class={cn(
		"rounded-button flex h-10 select-none items-center py-3 pl-3 pr-1.5 text-sm font-medium !ring-0 !ring-transparent data-[highlighted]:bg-muted",
		props.class,
	)}
>
	{@render children()}
</DropdownMenu.Item>
