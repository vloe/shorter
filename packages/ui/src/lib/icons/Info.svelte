<script lang="ts">
	import { cn } from "@sh/utils/src/cn"

	let className: String | undefined = undefined
	export { className as class }
</script>

<svg
	xmlns="http://www.w3.org/2000/svg"
	width="1em"
	height="1em"
	fill="currentColor"
	viewBox="0 0 256 256"
	class={cn("size-4", className)}
>
	<rect width="256" height="256" fill="none"></rect>
	<path
		d="M108,84a16,16,0,1,1,16,16A16,16,0,0,1,108,84Zm128,44A108,108,0,1,1,128,20,108.12,108.12,0,0,1,236,128Zm-24,0a84,84,0,1,0-84,84A84.09,84.09,0,0,0,212,128Zm-72,36.68V132a20,20,0,0,0-20-20,12,12,0,0,0-4,23.32V168a20,20,0,0,0,20,20,12,12,0,0,0,4-23.32Z"
	></path>
</svg>
