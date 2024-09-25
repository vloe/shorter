<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { Command as CommandPrimitive } from "cmdk-sv"

	type $$Props = CommandPrimitive.InputProps

	let className: null | string | undefined = undefined
	export { className as class }
	export let value: string = ""
</script>

<div class="flex items-center border-b px-3" data-cmdk-input-wrapper="">
	<CommandPrimitive.Input
		class={cx(
			"flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-muted-foreground disabled:cursor-not-allowed disabled:opacity-50",
			className,
		)}
		{...$$restProps}
		bind:value
	/>
</div>
