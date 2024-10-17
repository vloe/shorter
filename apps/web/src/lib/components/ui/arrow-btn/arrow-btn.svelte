<script lang="ts">
	import { Arrow } from "$lib/components/icons/arrow"
	import { cx } from "$lib/utils/cva.config"

	import { Btn, type BtnProps, btnVariants } from "../btn"

	let {
		builders,
		children,
		class: className,
		intent = "default",
		size = "default",
		...props
	}: BtnProps = $props()
</script>

<Btn
	{builders}
	class={cx(btnVariants({ className, intent, size }), "group gap-x-1.5 transition-all")}
	{...props}
	on:click
	on:keydown
>
	{@render children()}
	<Arrow class={cx("size-2.5 stroke-[1.5px]", className)} />
</Btn>
