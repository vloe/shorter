<script lang="ts">
	import { Button } from "bits-ui"
	import type { Snippet } from "svelte"
	import type { HTMLAnchorAttributes } from "svelte/elements"
	import type { VariantProps } from "cva"
	import { cva, cx } from "$lib/utils/cva.config"

	const btn = cva({
		base: "flex items-center justify-center whitespace-nowrap rounded-xl text-sm font-medium",
		variants: {
			intent: {
				primary: "bg-white text-black hover:bg-white/85",
				secondary: "w-min p-0 text-white hover:text-white/85",
				link: "text-primary underline-offset-4 hover:underline",
			},
			size: {
				default: "px-4 py-1.5",
				icon: "size-9",
			},
		},
	})

	type $Props = HTMLAnchorAttributes &
		VariantProps<typeof btn> & {
			children: Snippet
		}
	let {
		children,
		intent = "primary",
		size = "default",
		href = null,
		class: className,
	}: $Props = $props()
</script>

<Button.Root {href} class={cx(btn({ intent, size }), className)}>
	{@render children()}
</Button.Root>
