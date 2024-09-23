<script lang="ts">
	import { cx } from "$lib/utils/cva.config"
	import { Command as CommandPrimitive } from "cmdk-sv"

	type $Props = CommandPrimitive.InputProps

	let { children, class: className, value = $bindable(""), ...props }: $Props = $props()
</script>

<div class="flex items-center border-b px-3" data-cmdk-input-wrapper="">
	<CommandPrimitive.Input
		class={cx(
			"flex h-10 w-full rounded-md bg-transparent py-3 text-sm outline-none placeholder:text-white/40 disabled:cursor-not-allowed disabled:opacity-50",
			className,
		)}
		{...props}
		bind:value
	/>
</div>
