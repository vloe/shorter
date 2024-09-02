<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { Button as ButtonPrimitive } from "bits-ui"

	import { type BtnProps, btnVariants } from "./index"

	let {
		builders,
		children,
		class: className,
		intent = "default",
		size = "default",
		...props
	}: BtnProps = $props()
</script>

<ButtonPrimitive.Root
	{builders}
	class={cx(btnVariants({ className, intent, size }))}
	{...props}
	on:click
	on:keydown
>
	{@render children()}
</ButtonPrimitive.Root>
