import type { Snippet } from "svelte"
import type { HTMLAnchorAttributes } from "svelte/elements"

import { cva } from "$lib/utils/cva.config"
import { type VariantProps } from "cva"

import Link from "./link.svelte"

const linkVariants = cva({
	base: "inline-flex cursor-default items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring",
	variants: {
		intent: {
			ghost: "hover:bg-accent hover:text-accent-foreground",
			outline:
				"border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
			primary: "bg-primary text-primary-foreground shadow hover:bg-primary/85",
			secondary: "hover:text-primary/85",
		},
		size: {
			default: "h-9 px-4 py-2",
			icon: "h-9 w-9",
			lg: "h-10 rounded-md px-8",
			sm: "h-8 rounded-md px-3 text-xs",
		},
	},
})

type Variants = VariantProps<typeof linkVariants>
type Intent = Variants["intent"]
type Size = Variants["size"]

type LinkProps = {
	children: Snippet
	intent?: Intent
	size?: Size
} & HTMLAnchorAttributes

export { Link, type LinkProps, linkVariants }
