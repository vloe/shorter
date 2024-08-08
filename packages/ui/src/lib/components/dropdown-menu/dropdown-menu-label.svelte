<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "@sh/utils/src/cva.config"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $Props = {
		children: Snippet
		inset?: boolean
	} & DropdownMenuPrimitive.LabelProps

	let { children, class: className, inset, ...props }: $Props = $props()
</script>

<DropdownMenuPrimitive.Label
	class={cx("px-2 py-1.5 text-sm font-semibold", inset && "pl-8", className)}
	{...props}
>
	{@render children()}
</DropdownMenuPrimitive.Label>
