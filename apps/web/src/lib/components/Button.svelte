<script lang="ts">
	import { cn } from "$lib/utils/cn"
	import type { HTMLButtonAttributes } from "svelte/elements"
	import type { VariantProps } from "cva"
	import { btnVariants } from "$lib/utils/variants"

	interface $$Props extends HTMLButtonAttributes, VariantProps<typeof btnVariants> {}
	export let variant: $$Props["variant"] = "primary"
	export let size: $$Props["size"] = "default"
</script>

<button
	{...$$restProps}
	on:click|preventDefault
	class={cn(btnVariants({ variant, size }), $$props.class)}
>
	<slot />
</button>
