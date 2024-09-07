<script lang="ts">
	import { Arrow } from "$lib/components/icons/arrow"
	import { cx } from "$lib/utils/cva.config"
	import { Button as ButtonPrimitive } from "bits-ui"

	import { type BtnProps, btnVariants } from "../btn/index"

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
	class={cx(btnVariants({ className, intent, size }), "group gap-x-[5px] transition-all")}
	{...props}
	on:click
	on:keydown
>
	{@render children()}
	<Arrow class={cx("size-[10px] stroke-[1.5px]", className)} />
</ButtonPrimitive.Root>
