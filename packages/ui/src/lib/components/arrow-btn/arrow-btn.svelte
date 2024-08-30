<script lang="ts">
	import { cx } from "@sh/utils/src/cva.config"

	import Arrow from "../../icons/Arrow.svelte"
	import { Btn, type BtnProps, btnVariants } from "../btn/index"

	let {
		builders,
		children,
		class: className,
		intent = "primary",
		size = "default",
		...props
	}: BtnProps = $props()
</script>

<Btn
	{builders}
	class={cx(btnVariants({ className, intent, size }), "group gap-x-[5px] transition-all")}
	{...props}
	on:click
	on:keydown
>
	{@render children()}
	<Arrow />
</Btn>
