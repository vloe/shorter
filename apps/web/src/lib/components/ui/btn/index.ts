import type { WithElementRef } from "bits-ui"
import type { VariantProps } from "cva"
import type { HTMLAnchorAttributes, HTMLButtonAttributes } from "svelte/elements"

import { cva } from "$lib/utils/cva.config"

import Btn from "./btn.svelte"

const btnVariants = cva({
	base: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-lg text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
	defaultVariants: {
		size: "default",
		variant: "default",
	},
	variants: {
		size: {
			default: "h-8 px-4 py-2",
		},
		variant: {
			default: "bg-white text-black hover:bg-white/85",
			minimal: "hover:text-white/85",
		},
	},
})

type BtnVariant = VariantProps<typeof btnVariants>["variant"]
type BtnSize = VariantProps<typeof btnVariants>["size"]

type BtnProps = {
	size?: BtnSize
	variant?: BtnVariant
} & WithElementRef<HTMLAnchorAttributes> &
	WithElementRef<HTMLButtonAttributes>

export { Btn, type BtnProps, btnVariants }
