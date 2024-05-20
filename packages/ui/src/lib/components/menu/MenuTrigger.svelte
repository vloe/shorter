<script lang="ts">
	import { getCtx } from "$lib/components/menu/ctx"

	const {
		elements: { trigger },
	} = getCtx()
</script>

<button use:trigger>
	<slot />
</button>
