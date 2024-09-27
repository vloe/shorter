<script lang="ts">
	import { Arrow } from "$lib/components/icons/arrow"
	import { cx } from "$lib/utils/cva.config"
	import { Button as ButtonPrimitive } from "bits-ui"

	import { type ButtonProps, buttonVariants } from "../button"

	let {
		builders,
		class: className,
		size = "default",
		variant = "default",
		...props
	}: ButtonProps = $props()
</script>

<ButtonPrimitive.Root
	{builders}
	class={cx(buttonVariants({ className, size, variant }), "group gap-x-[5px] transition-all")}
	{...props}
	on:click
	on:keydown
>
	<slot />
	<Arrow class={cx("size-[10px] stroke-[1.5px]", className)} />
</ButtonPrimitive.Root>
