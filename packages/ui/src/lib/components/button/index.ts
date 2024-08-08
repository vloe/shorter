import type { Button as ButtonPrimitive } from "bits-ui";

import { cva } from  "@sh/utils/src/cva.config";
import { type VariantProps } from "cva";

import Root from "./button.svelte";

const buttonVariants = cva({
	base: "focus-visible:ring-ring inline-flex items-center justify-center whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 disabled:pointer-events-none disabled:opacity-50",
	defaultVariants: {
		size: "default",
		variant: "default",
	},
	variants: {
		size: {
			default: "h-9 px-4 py-2",
			icon: "h-9 w-9",
			lg: "h-10 rounded-md px-8",
			sm: "h-8 rounded-md px-3 text-xs",
		},
		variant: {
			default: "bg-primary text-primary-foreground hover:bg-primary/90 shadow",
			destructive:
				"bg-destructive text-destructive-foreground hover:bg-destructive/90 shadow-sm",
			ghost: "hover:bg-accent hover:text-accent-foreground",
			link: "text-primary underline-offset-4 hover:underline",
			outline:
				"border-input bg-background hover:bg-accent hover:text-accent-foreground border shadow-sm",
			secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80 shadow-sm",
		},
	},
});

type Variant = VariantProps<typeof buttonVariants>["variant"];
type Size = VariantProps<typeof buttonVariants>["size"];

type Props = {
	size?: Size;
	variant?: Variant;
} & ButtonPrimitive.Props;

type Events = ButtonPrimitive.Events;

export {
	buttonVariants,
	type Events,
	type Events as ButtonEvents,
	type Props,
	type Props as ButtonProps,
	Root,
	//
	Root as Button,
};
