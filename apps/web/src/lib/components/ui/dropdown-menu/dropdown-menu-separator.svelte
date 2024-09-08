<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $Props = DropdownMenuPrimitive.SeparatorProps

	let { class: className, ...props }: $Props = $props()
</script>

<DropdownMenuPrimitive.Separator class={cx("-mx-1 my-1 h-px bg-white/25", className)} {...props} />
