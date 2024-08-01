<script lang="ts">
	import { Button as ButtonPrimitive } from "bits-ui"
	import { type BtnProps, btnVariants } from "./index"
	import { cx } from "$lib/utils/cva.config"

	let {
		children,
		intent = "primary",
		size = "default",
		class: className,
		...props
	}: BtnProps = $props()
</script>

<ButtonPrimitive.Root
	class={cx(btnVariants({ intent, size, className }))}
	{...props}
	on:click
	on:keydown
>
	{@render children()}
</ButtonPrimitive.Root>
