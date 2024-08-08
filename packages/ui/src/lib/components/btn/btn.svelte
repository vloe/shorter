<script lang="ts">
	import { cx } from  "@sh/utils/src/cva.config";
	import { Button as ButtonPrimitive } from "bits-ui";

    import { type BtnProps, btnVariants } from "./index.js";

    let {
        builders,
        children,
		class: className,
		intent = "primary",
        size = "default",
		...props
    }: BtnProps = $props()
</script>

<ButtonPrimitive.Root
    {builders}
	class={cx(btnVariants({ className, intent, size }))}
	{...props}
	on:click
	on:keydown
>
    {@render children()}
</ButtonPrimitive.Root>
