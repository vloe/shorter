<script lang="ts">
	import { cn } from "$lib/utils/cn"
	import type { HTMLAnchorAttributes } from "svelte/elements"
	import type { VariantProps } from "cva"
	import { btnVariants } from "$lib/utils/variants"

	interface $$Props extends HTMLAnchorAttributes, VariantProps<typeof btnVariants> {}

	// props
	export let variant: $$Props["variant"] = "primary"
	export let size: $$Props["size"] = "default"
	export let href: $$Props["href"] = null
</script>

<a {...$$restProps} {href} class={cn(btnVariants({ variant, size }), $$props.class)}>
	<slot />
</a>
