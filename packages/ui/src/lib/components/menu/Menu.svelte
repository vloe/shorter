<script lang="ts">
	import { getCtx } from "$lib/components/menu/ctx"

	const {
		elements: { menu },
		states: { open },
	} = getCtx()
</script>

{#if $open}
	<div use:menu>
		<slot />
	</div>
{/if}
