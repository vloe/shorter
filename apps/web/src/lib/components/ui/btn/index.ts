import type { Button as ButtonPrimitive } from "bits-ui"
import type { VariantProps } from "cva"
import type { Snippet } from "svelte"

import { cva } from "$lib/utils/cva.config"

import Btn from "./btn.svelte"

const btnVariants = cva({
	base: "focus-visible:ring-ring inline-flex items-center justify-center whitespace-nowrap rounded-lg text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 disabled:pointer-events-none disabled:opacity-50",
	variants: {
		size: {
			default: "h-8 px-4",
			sm: "h-7 px-3.5",
		},
		intent: {
			default: "bg-white text-black shadow hover:bg-white/85",
			outline: "border bg-black hover:bg-white/15",
		},
	},
})

type Variant = VariantProps<typeof btnVariants>
type Intent = Variant["intent"]
type Size = Variant["size"]

type BtnProps = {
	children: Snippet
	size?: Size
	intent?: Intent
} & ButtonPrimitive.Props

export { btnVariants, type BtnProps, Btn }
