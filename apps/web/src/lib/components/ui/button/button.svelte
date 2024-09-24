<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { Button as ButtonPrimitive } from "bits-ui"

	import { buttonVariants, type Events, type Props } from "./index.js"

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
	class={cx(buttonVariants({ className, size, variant }))}
	type="button"
	{...$$restProps}
	on:click
	on:keydown
>
	<slot />
</ButtonPrimitive.Root>
