import type { Button as ButtonPrimitive } from "bits-ui"
import type { VariantProps } from "cva"
import type { Snippet } from "svelte"

import { cva } from "$lib/utils/cva.config"

import Btn from "./btn.svelte"

const btnVariants = cva({
	base: "inline-flex items-center justify-center whitespace-nowrap rounded-lg text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 disabled:pointer-events-none disabled:opacity-50",
	variants: {
		intent: {
			default: "bg-white text-black shadow hover:brightness-[85%]",
		},
		size: {
			default: "h-7 px-4 py-2",
		},
	},
})

type BtnVariants = VariantProps<typeof btnVariants>
type Intent = BtnVariants["intent"]
type Size = BtnVariants["size"]

type BtnProps = {
	children: Snippet
	intent?: Intent
	size?: Size
} & ButtonPrimitive.Props

export { Btn, type BtnProps, btnVariants }
