<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	type $$Props = HTMLAttributes<HTMLParagraphElement>

	let className: $$Props["class"] = undefined
	export { className as class }
</script>

<p class={cx("text-sm text-muted-foreground", className)} {...$$restProps}>
	<slot />
</p>
