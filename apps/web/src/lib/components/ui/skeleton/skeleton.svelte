<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"

	import { cx } from "$lib/utils/cva.config"

	type $Props = HTMLAttributes<HTMLDivElement>

	let { class: className, ...props }: $Props = $props()
</script>

<div class={cx("animate-pulse rounded-md bg-white/10", className)} {...props}></div>
