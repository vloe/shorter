<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { Button as ButtonPrimitive } from "bits-ui"

	import { type BtnProps, btnVariants } from "./index"

	let {
		children,
		class: className,
		intent = "primary",
		size = "default",
		...props
	}: BtnProps = $props()
</script>

<ButtonPrimitive.Root
	class={cx(btnVariants({ className, intent, size }))}
	{...props}
	on:click
	on:keydown
>
	{@render children()}
</ButtonPrimitive.Root>
