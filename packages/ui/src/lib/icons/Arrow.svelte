<script lang="ts">
	import { cn } from "../utils/cn"

	interface $Props {
		class?: string
	}
	let { class: className }: $Props = $props()
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	viewBox="0 0 10 10"
	stroke="currentColor"
	fill="none"
	aria-hidden="true"
	stroke-width="2"
	class={cn("h-5 w-5", className)}
>
	<path d="M0 5h7" class="delay-50 opacity-0 transition-opacity group-hover:opacity-100" />
	<path
		d="M1 1l4 4-4 4"
		class="transition duration-200 ease-in-out group-hover:translate-x-[3px]"
	/>
</svg>
