<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	import type { HeadingLevel } from "./index.js"

	type $$Props = {
		tag?: HeadingLevel
	} & HTMLAttributes<HTMLHeadingElement>

	let className: $$Props["class"] = undefined
	export let tag: $$Props["tag"] = "h3"
	export { className as class }
</script>

<svelte:element
	class={cx("font-semibold leading-none tracking-tight", className)}
	this={tag}
	{...$$restProps}
>
	<slot />
</svelte:element>
