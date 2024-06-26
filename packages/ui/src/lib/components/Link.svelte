<script lang="ts">
	import { cn } from "../utils/cn"
	import { cva, type VariantProps } from "cva"
	import type { Snippet } from "svelte"
	import type { HTMLAnchorAttributes } from "svelte/elements"

	const variants = cva(
		"inline-flex cursor-pointer items-center justify-center whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
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
					xs: "h-7 w-7 rounded-md",
					sm: "h-9 rounded-md px-3",
					lg: "h-11 rounded-md px-8",
					icon: "h-10 w-10",
				},
			},
		},
	)

	interface $Props extends HTMLAnchorAttributes, VariantProps<typeof variants> {
		children: Snippet
		href: string
	}
	let { children, variant = "link", size = "default", href, ...props }: $Props = $props()
</script>

<a {href} {...props} class={cn(variants({ variant, size }), props.class)}>
	{@render children()}
</a>
