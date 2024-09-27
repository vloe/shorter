<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $$Props = {
		inset?: boolean
	} & DropdownMenuPrimitive.LabelProps

	let className: $$Props["class"] = undefined
	export let inset: $$Props["inset"] = undefined
	export { className as class }
</script>

<DropdownMenuPrimitive.Label
	class={cx("px-2 py-1.5 text-sm font-semibold", inset && "pl-8", className)}
	{...$$restProps}
>
	<slot />
</DropdownMenuPrimitive.Label>
