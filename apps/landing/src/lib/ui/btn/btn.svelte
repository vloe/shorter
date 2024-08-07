<script lang="ts">
<<<<<<< HEAD
	import { cx } from "$lib/utils/cva.config"
	import { Button as ButtonPrimitive } from "bits-ui"

	import { type BtnProps, btnVariants } from "./index"

	let {
		children,
		class: className,
		intent = "primary",
		size = "default",
=======
	import { Button as ButtonPrimitive } from "bits-ui"
	import { type BtnProps, btnVariants } from "./index"
	import { cx } from "$lib/utils/cva.config"

	let {
		children,
		intent = "primary",
		size = "default",
		class: className,
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
		...props
	}: BtnProps = $props()
</script>

<ButtonPrimitive.Root
<<<<<<< HEAD
	class={cx(btnVariants({ className, intent, size }))}
=======
	class={cx(btnVariants({ intent, size, className }))}
>>>>>>> 1d6c46b6adb216cc7581a242ef7bf92c418e600c
	{...props}
	on:click
	on:keydown
>
	{@render children()}
</ButtonPrimitive.Root>
