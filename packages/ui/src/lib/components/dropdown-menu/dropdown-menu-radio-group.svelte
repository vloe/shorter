<script lang="ts">
	import type { Snippet } from "svelte"

	import { DropdownMenu as DropdownMenuPrimitive } from "bits-ui"

	type $Props = {
		children: Snippet
	} & DropdownMenuPrimitive.RadioGroupProps

	let { children, value, ...props }: $Props = $props()
</script>

<DropdownMenuPrimitive.RadioGroup {...props} bind:value>
	{@render children()}
</DropdownMenuPrimitive.RadioGroup>
