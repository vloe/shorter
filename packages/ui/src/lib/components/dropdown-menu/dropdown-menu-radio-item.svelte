<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "@sh/utils/src/cva.config"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"
	import DotFilled from "svelte-radix/DotFilled.svelte"

	type $Props = {
		children: Snippet
	} & DropdownMenuPrimitive.RadioItemProps

	let { children, class: className, value, ...props }: $Props = $props()
</script>

<DropdownMenuPrimitive.RadioItem
	class={cx(
		"relative flex cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none data-[disabled]:pointer-events-none data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground data-[disabled]:opacity-50",
		className,
	)}
	{value}
	{...props}
	on:click
	on:focusin
	on:focusout
	on:keydown
	on:pointerdown
	on:pointerleave
	on:pointermove
>
	<span class="absolute left-2 flex h-3.5 w-3.5 items-center justify-center">
		<DropdownMenuPrimitive.RadioIndicator>
			<DotFilled class="h-4 w-4 fill-current" />
		</DropdownMenuPrimitive.RadioIndicator>
	</span>
	{@render children()}
</DropdownMenuPrimitive.RadioItem>
