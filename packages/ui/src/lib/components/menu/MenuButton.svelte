<script lang="ts">
	import type { HTMLButtonAttributes } from "svelte/elements"
	import { getCtx } from "$lib/components/menu/ctx"

	const {
		elements: { item },
	} = getCtx()

	interface $$Props extends HTMLButtonAttributes {}
</script>

<button {...$$restProps} on:click use:item>
	<slot />
</button>
