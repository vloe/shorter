<script lang="ts">
	import { cx } from "$lib/utils/cva.config"

	import { type LinkProps, linkVariants } from "../link/index"

	let {
		children,
		class: className,
		intent = "primary",
		size = "default",
		...props
	}: LinkProps = $props()
</script>

<a
	class={cx(linkVariants({ className, intent, size }), "group gap-x-[5px] transition-all")}
	{...props}
>
	{@render children()}
	<svg
		class={cx("size-[10px] stroke-[1.5px]", className)}
		fill="none"
		stroke="currentColor"
		viewBox="0 0 10 10"
		xmlns="http://www.w3.org/2000/svg"
	>
		<path class="opacity-0 transition-opacity group-hover:opacity-100" d="M0 5h7" />
		<path class="transition-all ease-in-out group-hover:translate-x-[2px]" d="M1 1l4 4-4 4" />
	</svg>
</a>
