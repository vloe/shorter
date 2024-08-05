import type { Button as ButtonPrimitive } from "bits-ui"
import type { Snippet } from "svelte"
import type { VariantProps } from "cva"
import { cva } from "$lib/utils/cva.config"
import Btn from "./btn.svelte"

const btnVariants = cva({
	base: "inline-flex items-center justify-center whitespace-nowrap rounded-lg text-sm font-medium transition-colors focus-visible:outline-none",
	variants: {
		intent: {
			primary: "bg-white text-black shadow hover:bg-white/85",
			secondary: "text-white hover:text-white/85",
			ghost: "text-white hover:bg-white/30",
		},
		size: {
			default: "px-3.5 py-1.5",
			icon: "rounded-xl px-2.5 py-1.5",
		},
	},
})

type Variants = VariantProps<typeof btnVariants>
type Intent = Variants["intent"]
type Size = Variants["size"]

type BtnProps = ButtonPrimitive.Props & {
	children: Snippet
	intent?: Intent
	size?: Size
}

export { Btn, type BtnProps, btnVariants }
