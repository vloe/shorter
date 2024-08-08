import type { Button as ButtonPrimitive } from "bits-ui"
import type { Snippet } from "svelte"

import { cva } from "@sh/utils/src/cva.config"
import { type VariantProps } from "cva"

import Btn from "./btn.svelte"

const btnVariants = cva({
	base: "inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50",
	variants: {
		intent: {
			destructive:
				"bg-destructive text-destructive-foreground shadow-sm hover:bg-destructive/90",
			ghost: "hover:bg-accent hover:text-accent-foreground",
			link: "text-primary underline-offset-4 hover:underline",
			outline:
				"border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground",
			primary: "bg-primary text-primary-foreground shadow hover:bg-primary/90",
			secondary: "bg-secondary text-secondary-foreground shadow-sm hover:bg-secondary/80",
		},
		size: {
			default: "h-9 px-4 py-2",
			icon: "h-9 w-9",
			lg: "h-10 rounded-md px-8",
			sm: "h-8 rounded-md px-3 text-xs",
		},
	},
})

type Variants = VariantProps<typeof btnVariants>
type Intent = Variants["intent"]
type Size = Variants["size"]

type BtnProps = {
	children: Snippet
	intent?: Intent
	size?: Size
} & ButtonPrimitive.Props

export { Btn, type BtnProps, btnVariants }
