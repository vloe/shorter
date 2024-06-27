<script lang="ts">
	import { cn } from "../utils/cn"
	import { btnVariants } from "../utils/variants"
	import { Button } from "bits-ui"
	import type { Button as ButtonPrimitive } from "bits-ui"
	import type { VariantProps } from "cva"
	import type { Snippet } from "svelte"

	type $Props = ButtonPrimitive.Props & {
		children: Snippet
		variant?: VariantProps<typeof btnVariants>["variant"]
		size?: VariantProps<typeof btnVariants>["size"]
	}

	let { children, variant = "primary", size = "default", ...props }: $Props = $props()
</script>

<Button.Root {...props} class={cn(btnVariants({ variant, size }), props.class)}>
	{@render children()}
</Button.Root>
