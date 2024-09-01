<script lang="ts">
	import { cn } from "../../../utils"

	type $Props = {
		class?: string
	}

	let { class: className }: $Props = $props()
</script>

<svg
	class={cn("size-6", className)}
	fill="currentColor"
	viewBox="0 0 16 10"
	xmlns="http://www.w3.org/2000/svg"
>
	<rect height="2" rx="1" width="16" y="8" />
	<rect height="2" rx="1" width="16" y="4" />
	<rect height="2" rx="1" width="16" />
</svg>
