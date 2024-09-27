<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	type $$Props = HTMLAttributes<HTMLDivElement>

	let className: $$Props["class"] = undefined
	export { className as class }
</script>

<div class={cx("flex flex-col space-y-1.5 p-6 pb-0", className)} {...$$restProps}>
	<slot />
</div>
