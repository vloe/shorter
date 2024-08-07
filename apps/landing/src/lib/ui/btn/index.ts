import type { Button as ButtonPrimitive } from "bits-ui"
<<<<<<< HEAD
import type { VariantProps } from "cva"
import type { Snippet } from "svelte"

import { cva } from "$lib/utils/cva.config"

=======
import type { Snippet } from "svelte"
import type { VariantProps } from "cva"
import { cva } from "$lib/utils/cva.config"
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
import Btn from "./btn.svelte"

const btnVariants = cva({
	base: "inline-flex items-center justify-center whitespace-nowrap rounded-lg text-sm font-medium transition-colors focus-visible:outline-none",
	variants: {
		intent: {
<<<<<<< HEAD
			ghost: "text-white hover:bg-white/30",
			gradient: "btn-gradient border border-shpurple text-white",
			primary: "bg-white text-black shadow hover:bg-white/85",
			secondary: "text-white hover:text-white/85",
=======
			primary: "bg-white text-black shadow hover:bg-white/85",
			secondary: "text-white hover:text-white/85",
			ghost: "text-white hover:bg-white/30",
			gradient: "btn-gradient border border-shpurple text-white",
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
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

<<<<<<< HEAD
type BtnProps = {
	children: Snippet
	intent?: Intent
	size?: Size
} & ButtonPrimitive.Props
=======
type BtnProps = ButtonPrimitive.Props & {
	children: Snippet
	intent?: Intent
	size?: Size
}
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c

export { Btn, type BtnProps, btnVariants }
