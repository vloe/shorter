<script lang="ts">
	import { Button as ButtonPrimitive } from "bits-ui"

	import { cn } from "../../../utils"
	import { buttonVariants, type Events, type Props } from "../button/index.js"

	type $$Props = Props
	type $$Events = Events

	let className: $$Props["class"] = undefined
	export let variant: $$Props["variant"] = "default"
	export let size: $$Props["size"] = "default"
	export let builders: $$Props["builders"] = []
	export { className as class }
</script>

<ButtonPrimitive.Root
	{builders}
	class={cn(buttonVariants({ className, size, variant }), "group gap-x-[5px] transition-all")}
	type="button"
	{...$$restProps}
	on:click
	on:keydown
>
	<slot />
	<svg
		class={cn("size-[10px] stroke-[1.5px]", className)}
		fill="none"
		stroke="currentColor"
		viewBox="0 0 10 10"
		xmlns="http://www.w3.org/2000/svg"
	>
		<path class="opacity-0 transition-opacity group-hover:opacity-100" d="M0 5h7" />
		<path class="transition-all ease-in-out group-hover:translate-x-[2px]" d="M1 1l4 4-4 4" />
	</svg>
</ButtonPrimitive.Root>
