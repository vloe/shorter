<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	import { type BtnProps, btnVariants } from "./index"

	let {
		children,
		class: className,
		href = undefined,
		ref = $bindable(null),
		size = "default",
		type = "button",
		variant = "default",
		...restProps
	}: BtnProps = $props()
</script>

{#if href}
	<a bind:this={ref} class={cx(btnVariants({ className, size, variant }))} {href} {...restProps}>
		{@render children?.()}
	</a>
{:else}
	<button
		bind:this={ref}
		class={cx(btnVariants({ className, size, variant }))}
		{type}
		{...restProps}
	>
		{@render children?.()}
	</button>
{/if}
