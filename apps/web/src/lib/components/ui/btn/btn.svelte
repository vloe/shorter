<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { Button as ButtonPrimitive } from "bits-ui"

	import { btnVariants, type BtnProps } from "./index.js"

	let {
		class: className,
		children,
		size = "default",
		intent = "default",
		builders,
		...props
	}: BtnProps = $props()
</script>

<ButtonPrimitive.Root
	{builders}
	class={cx(btnVariants({ className, size, intent }))}
	{...props}
	on:click
	on:keydown
>
	{@render children()}
</ButtonPrimitive.Root>
