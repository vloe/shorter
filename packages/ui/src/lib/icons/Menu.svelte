<script lang="ts">
	import { cn } from "../utils/cn"

	interface $Props {
		class?: string
	}
	let { class: className }: $Props = $props()
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	viewBox="0 0 24 12"
	fill="currentColor"
	class={cn("size-6", className)}
>
	<rect width="24" height="2" />
	<rect width="24" y="5" height="2" />
	<rect width="24" y="10" height="2" />
</svg>
