<script lang="ts">
	import { Arrow } from "$lib/components/icons/arrow"
	import { cx } from "$lib/utils/cva.config"

	import { type BtnProps, btnVariants, Btn } from "../btn"

	let {
		children,
		builders,
		class: className,
		size = "default",
		intent = "default",
		...props
	}: BtnProps = $props()
</script>

<Btn
	{builders}
	class={cx(btnVariants({ className, size, intent }), "group gap-x-1.5 transition-all")}
	{...props}
	on:click
	on:keydown
>
	{@render children()}
	<Arrow class={cx("size-2.5 stroke-[1.5px]", className)} />
</Btn>
