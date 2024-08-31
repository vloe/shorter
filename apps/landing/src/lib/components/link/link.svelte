<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	import { type LinkProps, linkVariants } from "./index"

	let {
		children,
		class: className,
		intent = "primary",
		size = "default",
		...props
	}: LinkProps = $props()
</script>

<a class={cx(linkVariants({ className, intent, size }))} {...props}>
	{@render children()}
</a>
