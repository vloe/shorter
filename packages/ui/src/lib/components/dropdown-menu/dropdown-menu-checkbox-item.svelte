<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "@sh/utils/src/cva.config"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"
	import Check from "svelte-radix/Check.svelte"

	type $Props = {
		children: Snippet
	} & DropdownMenuPrimitive.CheckboxItemProps

	let { checked, children, class: className, ...props }: $Props = $props()
</script>

<DropdownMenuPrimitive.CheckboxItem
	bind:checked
	class={cx(
		"relative flex cursor-default select-none items-center rounded-sm py-1.5 pl-8 pr-2 text-sm outline-none data-[disabled]:pointer-events-none data-[highlighted]:bg-accent data-[highlighted]:text-accent-foreground data-[disabled]:opacity-50",
		className,
	)}
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
		<DropdownMenuPrimitive.CheckboxIndicator>
			<Check class="h-4 w-4" />
		</DropdownMenuPrimitive.CheckboxIndicator>
	</span>
	{@render children()}
</DropdownMenuPrimitive.CheckboxItem>
