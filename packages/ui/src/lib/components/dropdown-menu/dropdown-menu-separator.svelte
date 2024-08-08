<script lang="ts">
	import type { Snippet } from "svelte"

	import { cx } from "@sh/utils/src/cva.config"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $Props = {
		children: Snippet
	} & DropdownMenuPrimitive.SeparatorProps

	let { class: className, ...props }: $Props = $props()
</script>

<DropdownMenuPrimitive.Separator class={cx("-mx-1 my-1 h-px bg-muted", className)} {...props} />
