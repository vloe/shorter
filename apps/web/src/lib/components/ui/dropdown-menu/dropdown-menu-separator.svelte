<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $$Props = DropdownMenuPrimitive.SeparatorProps

	let className: $$Props["class"] = undefined
	export { className as class }
</script>

<DropdownMenuPrimitive.Separator
	class={cx("-mx-1 my-1 h-px bg-muted", className)}
	{...$$restProps}
/>
