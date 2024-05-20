<script lang="ts">
	import type { HTMLAnchorAttributes } from "svelte/elements"
	import { getCtx } from "$lib/components/menu/ctx"

	const {
		elements: { item },
	} = getCtx()

	interface $$Props extends HTMLAnchorAttributes {}
	export let href: $$Props["href"] = null
</script>

<a {...$$restProps} {href} use:item>
	<slot />
</a>
