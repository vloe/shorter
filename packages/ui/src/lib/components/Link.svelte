<script lang="ts">
	import { cn } from "$lib/utils/cn.ts"
	import { cva } from "cva"
	import type { HTMLAnchorAttributes } from "svelte/elements"
	import type { VariantProps } from "cva"

	const linkVariants = cva(
		"inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
		{
			variants: {
				variant: {
					primary: "bg-primary text-primary-foreground hover:bg-primary/90",
					secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
					outline:
						"border border-input bg-background hover:bg-accent hover:text-accent-foreground",
					ghost: "hover:bg-accent hover:text-accent-foreground",
					link: "text-primary underline-offset-4 hover:underline",
				},
				size: {
					default: "h-10 px-4 py-2",
					sm: "h-9 rounded-md px-3",
					lg: "h-11 rounded-md px-8",
					icon: "h-9 w-9",
				},
			},
		},
	)

	interface $$Props extends HTMLAnchorAttributes, VariantProps<typeof linkVariants> {}
	export let variant: $$Props["variant"] = "link"
	export let size: $$Props["size"] = "default"
	export let href: $$Props["href"]
</script>

<a {...$$restProps} {href} class={cn(linkVariants({ variant, size }), $$props.class)}>
	<slot />
</a>
