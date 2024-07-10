<script lang="ts">
	import type { HTMLAttributes } from "svelte/elements"
	import { cn } from "@sh/utils/src/cn"

	type $$Props = HTMLAttributes<HTMLTableSectionElement>

	let className: $$Props["class"] = undefined
	export { className as class }
</script>

<tbody class={cn("[&_tr:last-child]:border-0", className)} {...$$restProps}>
	<slot />
</tbody>
